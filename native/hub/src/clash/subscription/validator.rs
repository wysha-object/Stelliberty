// 订阅配置验证模块
//
// 提供详细的 Clash 配置文件验证功能
// 包括 YAML 语法、必需字段、代理配置、规则语法等全面验证

#![allow(clippy::needless_borrows_for_generic_args)]
#![allow(clippy::needless_borrow)]

use rinf::{DartSignal, RustSignal};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

// Dart → Rust: 验证订阅配置请求
#[derive(Deserialize, DartSignal)]
pub struct ValidateSubscriptionRequest {
    pub content: String,
}

// Rust → Dart: 验证结果响应
#[derive(Serialize, RustSignal)]
pub struct ValidateSubscriptionResponse {
    pub is_valid: bool,
    pub error_message: Option<String>, // 简单的错误提示,给用户看的
}

impl ValidateSubscriptionRequest {
    // 处理验证请求
    pub fn handle(self) {
        log::debug!("开始验证订阅配置（长度：{} 字符）", self.content.len());

        let response = match validate_clash_config(&self.content) {
            Ok(()) => {
                log::info!("订阅配置验证通过");
                ValidateSubscriptionResponse {
                    is_valid: true,
                    error_message: None,
                }
            }
            Err(errors) => {
                // 打印所有验证错误
                log::error!("订阅配置验证失败，共 {} 个错误", errors.len());

                // 打印所有错误的详细信息
                for (i, err) in errors.iter().enumerate() {
                    let field_info = if let Some(field) = &err.field {
                        format!(" [{}]", field)
                    } else {
                        String::new()
                    };
                    log::error!(
                        "  {}. {}{}: {}",
                        i + 1,
                        err.category,
                        field_info,
                        err.message
                    );
                }

                // Dart 端只返回简单的错误提示
                ValidateSubscriptionResponse {
                    is_valid: false,
                    error_message: Some("配置文件格式不正确".to_string()),
                }
            }
        };

        response.send_signal_to_dart();
    }
}

// 验证错误详情（内部类型，不导出到 Dart）
#[derive(Clone)]
struct ValidationError {
    category: String,      // 错误类别（如 "YAML语法", "代理配置"）
    field: Option<String>, // 相关字段名（如 "proxies[0].name"）
    message: String,       // 错误描述
}

// 验证 Clash 配置文件
fn validate_clash_config(content: &str) -> Result<(), Vec<ValidationError>> {
    let mut errors = Vec::new();

    // 1. 验证 YAML 语法
    let doc = match serde_yaml_ng::from_str::<serde_yaml_ng::Value>(content) {
        Ok(d) => d,
        Err(e) => {
            errors.push(ValidationError {
                category: "YAML语法".to_string(),
                field: None,
                message: format!("YAML 格式错误：{}", e),
            });
            return Err(errors);
        }
    };

    // 确保根节点是对象
    let root = match doc.as_mapping() {
        Some(m) => m,
        None => {
            errors.push(ValidationError {
                category: "配置结构".to_string(),
                field: None,
                message: "配置文件根节点必须是对象".to_string(),
            });
            return Err(errors);
        }
    };

    // 2. 验证必需字段
    if !root.contains_key(&serde_yaml_ng::Value::String("proxies".to_string())) {
        errors.push(ValidationError {
            category: "必需字段".to_string(),
            field: Some("proxies".to_string()),
            message: "缺少必需字段：proxies".to_string(),
        });
    }

    if !root.contains_key(&serde_yaml_ng::Value::String("proxy-groups".to_string())) {
        errors.push(ValidationError {
            category: "必需字段".to_string(),
            field: Some("proxy-groups".to_string()),
            message: "缺少必需字段：proxy-groups".to_string(),
        });
    }

    // 如果缺少必需字段，直接返回
    if !errors.is_empty() {
        return Err(errors);
    }

    // 3. 验证 proxies 字段并收集代理名称
    let proxy_names = match validate_proxies(&root) {
        Ok(names) => names,
        Err(mut proxy_errors) => {
            errors.append(&mut proxy_errors);
            HashSet::new() // 如果验证失败，返回空集合
        }
    };

    // 4. 验证 proxy-groups 字段并检查引用的代理是否存在
    let group_names = match validate_proxy_groups(&root, &proxy_names) {
        Ok(names) => names,
        Err(mut group_errors) => {
            errors.append(&mut group_errors);
            HashSet::new()
        }
    };

    // 5. 验证 rules 字段（如果存在），检查引用的代理组是否存在
    if let Err(mut rule_errors) = validate_rules(&root, &group_names, &proxy_names) {
        errors.append(&mut rule_errors);
    }

    // 6. 检查循环引用（代理组之间）
    if let Err(mut cycle_errors) = check_group_cycles(&root) {
        errors.append(&mut cycle_errors);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

// 验证 proxies 字段
fn validate_proxies(
    root: &serde_yaml_ng::Mapping,
) -> Result<HashSet<String>, Vec<ValidationError>> {
    let mut errors = Vec::new();

    let proxies = match root.get(&serde_yaml_ng::Value::String("proxies".to_string())) {
        Some(p) => p,
        None => return Ok(HashSet::new()), // 已在上层验证
    };

    let proxies_array = match proxies.as_sequence() {
        Some(a) => a,
        None => {
            errors.push(ValidationError {
                category: "代理配置".to_string(),
                field: Some("proxies".to_string()),
                message: "proxies 必须是数组".to_string(),
            });
            return Err(errors);
        }
    };

    if proxies_array.is_empty() {
        errors.push(ValidationError {
            category: "代理配置".to_string(),
            field: Some("proxies".to_string()),
            message: "proxies 数组不能为空".to_string(),
        });
        return Err(errors);
    }

    // 验证每个代理节点
    let mut proxy_names = HashSet::new();
    for (i, proxy) in proxies_array.iter().enumerate() {
        let proxy_obj = match proxy.as_mapping() {
            Some(obj) => obj,
            None => {
                errors.push(ValidationError {
                    category: "代理配置".to_string(),
                    field: Some(format!("proxies[{}]", i)),
                    message: "代理节点必须是对象".to_string(),
                });
                continue;
            }
        };

        // 验证 name 字段
        let name = match proxy_obj.get(&serde_yaml_ng::Value::String("name".to_string())) {
            Some(n) => match n.as_str() {
                Some(s) => {
                    // 检查名称是否为空
                    if s.trim().is_empty() {
                        errors.push(ValidationError {
                            category: "代理配置".to_string(),
                            field: Some(format!("proxies[{}].name", i)),
                            message: "代理名称不能为空".to_string(),
                        });
                        continue;
                    }
                    s
                }
                None => {
                    errors.push(ValidationError {
                        category: "代理配置".to_string(),
                        field: Some(format!("proxies[{}].name", i)),
                        message: "name 必须是字符串".to_string(),
                    });
                    continue;
                }
            },
            None => {
                errors.push(ValidationError {
                    category: "代理配置".to_string(),
                    field: Some(format!("proxies[{}]", i)),
                    message: "缺少必需字段：name".to_string(),
                });
                continue;
            }
        };

        // 检查代理名称重复
        if !proxy_names.insert(name.to_string()) {
            errors.push(ValidationError {
                category: "代理配置".to_string(),
                field: Some(format!("proxies[{}].name", i)),
                message: format!("代理名称重复：{}", name),
            });
        }

        // 验证 type 字段
        let proxy_type = match proxy_obj.get(&serde_yaml_ng::Value::String("type".to_string())) {
            Some(t) => match t.as_str() {
                Some(s) => s,
                None => {
                    errors.push(ValidationError {
                        category: "代理配置".to_string(),
                        field: Some(format!("proxies[{}].type", i)),
                        message: "type 必须是字符串".to_string(),
                    });
                    continue;
                }
            },
            None => {
                errors.push(ValidationError {
                    category: "代理配置".to_string(),
                    field: Some(format!("proxies[{}]", i)),
                    message: "缺少必需字段：type".to_string(),
                });
                continue;
            }
        };

        // 验证代理类型
        const VALID_PROXY_TYPES: &[&str] = &[
            "ss",
            "ssr",
            "vmess",
            "vless",
            "trojan",
            "hysteria",
            "hysteria2",
            "tuic",
            "wireguard",
            "socks5",
            "http",
            "snell",
        ];
        if !VALID_PROXY_TYPES.contains(&proxy_type) {
            errors.push(ValidationError {
                category: "代理配置".to_string(),
                field: Some(format!("proxies[{}].type", i)),
                message: format!("不支持的代理类型：{}", proxy_type),
            });
            continue;
        }

        // 验证 server 字段（除了 wireguard 都需要）
        if proxy_type != "wireguard" {
            match proxy_obj.get(&serde_yaml_ng::Value::String("server".to_string())) {
                Some(server) => {
                    if let Some(server_str) = server.as_str() {
                        if server_str.trim().is_empty() {
                            errors.push(ValidationError {
                                category: "代理配置".to_string(),
                                field: Some(format!("proxies[{}].server", i)),
                                message: "服务器地址不能为空".to_string(),
                            });
                        }
                    } else {
                        errors.push(ValidationError {
                            category: "代理配置".to_string(),
                            field: Some(format!("proxies[{}].server", i)),
                            message: "server 必须是字符串".to_string(),
                        });
                    }
                }
                None => {
                    errors.push(ValidationError {
                        category: "代理配置".to_string(),
                        field: Some(format!("proxies[{}]", i)),
                        message: "缺少必需字段：server".to_string(),
                    });
                }
            }
        }

        // 验证 port 字段（除了 wireguard 都需要）
        if proxy_type != "wireguard" {
            match proxy_obj.get(&serde_yaml_ng::Value::String("port".to_string())) {
                Some(p) => {
                    if let Some(port_num) = p.as_i64() {
                        if !(1..=65535).contains(&port_num) {
                            errors.push(ValidationError {
                                category: "代理配置".to_string(),
                                field: Some(format!("proxies[{}].port", i)),
                                message: format!("端口号超出有效范围：{}", port_num),
                            });
                        }
                    } else {
                        errors.push(ValidationError {
                            category: "代理配置".to_string(),
                            field: Some(format!("proxies[{}].port", i)),
                            message: "port 必须是数字".to_string(),
                        });
                    }
                }
                None => {
                    errors.push(ValidationError {
                        category: "代理配置".to_string(),
                        field: Some(format!("proxies[{}]", i)),
                        message: "缺少必需字段：port".to_string(),
                    });
                }
            }
        }

        // 根据代理类型验证特定字段
        match proxy_type {
            "ss" | "ssr" => {
                // 验证 cipher/password
                if !proxy_obj.contains_key(&serde_yaml_ng::Value::String("cipher".to_string())) {
                    errors.push(ValidationError {
                        category: "代理配置".to_string(),
                        field: Some(format!("proxies[{}]", i)),
                        message: format!("{} 代理缺少必需字段：cipher", proxy_type),
                    });
                }
                if !proxy_obj.contains_key(&serde_yaml_ng::Value::String("password".to_string())) {
                    errors.push(ValidationError {
                        category: "代理配置".to_string(),
                        field: Some(format!("proxies[{}]", i)),
                        message: format!("{} 代理缺少必需字段：password", proxy_type),
                    });
                }
            }
            "vmess" | "vless" => {
                // 验证 uuid
                if !proxy_obj.contains_key(&serde_yaml_ng::Value::String("uuid".to_string())) {
                    errors.push(ValidationError {
                        category: "代理配置".to_string(),
                        field: Some(format!("proxies[{}]", i)),
                        message: format!("{} 代理缺少必需字段：uuid", proxy_type),
                    });
                }
            }
            "trojan" | "hysteria" | "hysteria2" => {
                // 验证 password
                if !proxy_obj.contains_key(&serde_yaml_ng::Value::String("password".to_string())) {
                    errors.push(ValidationError {
                        category: "代理配置".to_string(),
                        field: Some(format!("proxies[{}]", i)),
                        message: format!("{} 代理缺少必需字段：password", proxy_type),
                    });
                }
            }
            _ => {}
        }
    }

    if errors.is_empty() {
        Ok(proxy_names)
    } else {
        Err(errors)
    }
}

// 验证 proxy-groups 字段
fn validate_proxy_groups(
    root: &serde_yaml_ng::Mapping,
    proxy_names: &HashSet<String>,
) -> Result<HashSet<String>, Vec<ValidationError>> {
    let mut errors = Vec::new();

    let groups = match root.get(&serde_yaml_ng::Value::String("proxy-groups".to_string())) {
        Some(g) => g,
        None => return Ok(HashSet::new()), // 已在上层验证
    };

    let groups_array = match groups.as_sequence() {
        Some(a) => a,
        None => {
            errors.push(ValidationError {
                category: "代理组配置".to_string(),
                field: Some("proxy-groups".to_string()),
                message: "proxy-groups 必须是数组".to_string(),
            });
            return Err(errors);
        }
    };

    if groups_array.is_empty() {
        errors.push(ValidationError {
            category: "代理组配置".to_string(),
            field: Some("proxy-groups".to_string()),
            message: "proxy-groups 数组不能为空".to_string(),
        });
        return Err(errors);
    }

    // 第一阶段：收集所有代理组名称
    let mut group_names = HashSet::new();
    for (i, group) in groups_array.iter().enumerate() {
        let group_obj = match group.as_mapping() {
            Some(obj) => obj,
            None => {
                errors.push(ValidationError {
                    category: "代理组配置".to_string(),
                    field: Some(format!("proxy-groups[{}]", i)),
                    message: "代理组必须是对象".to_string(),
                });
                continue;
            }
        };

        // 验证 name 字段
        let name = match group_obj.get(&serde_yaml_ng::Value::String("name".to_string())) {
            Some(n) => match n.as_str() {
                Some(s) => {
                    if s.trim().is_empty() {
                        errors.push(ValidationError {
                            category: "代理组配置".to_string(),
                            field: Some(format!("proxy-groups[{}].name", i)),
                            message: "代理组名称不能为空".to_string(),
                        });
                        continue;
                    }
                    s
                }
                None => {
                    errors.push(ValidationError {
                        category: "代理组配置".to_string(),
                        field: Some(format!("proxy-groups[{}].name", i)),
                        message: "name 必须是字符串".to_string(),
                    });
                    continue;
                }
            },
            None => {
                errors.push(ValidationError {
                    category: "代理组配置".to_string(),
                    field: Some(format!("proxy-groups[{}]", i)),
                    message: "缺少必需字段：name".to_string(),
                });
                continue;
            }
        };

        // 检查代理组名称重复
        if !group_names.insert(name.to_string()) {
            errors.push(ValidationError {
                category: "代理组配置".to_string(),
                field: Some(format!("proxy-groups[{}].name", i)),
                message: format!("代理组名称重复：{}", name),
            });
        }
    }

    // 第二阶段：验证每个代理组的详细配置
    for (i, group) in groups_array.iter().enumerate() {
        let group_obj = match group.as_mapping() {
            Some(obj) => obj,
            None => continue, // 第一阶段已报错
        };

        // 验证 type 字段
        let group_type = match group_obj.get(&serde_yaml_ng::Value::String("type".to_string())) {
            Some(t) => match t.as_str() {
                Some(s) => s,
                None => {
                    errors.push(ValidationError {
                        category: "代理组配置".to_string(),
                        field: Some(format!("proxy-groups[{}].type", i)),
                        message: "type 必须是字符串".to_string(),
                    });
                    continue;
                }
            },
            None => {
                errors.push(ValidationError {
                    category: "代理组配置".to_string(),
                    field: Some(format!("proxy-groups[{}]", i)),
                    message: "缺少必需字段：type".to_string(),
                });
                continue;
            }
        };

        // 验证代理组类型
        const VALID_GROUP_TYPES: &[&str] =
            &["select", "url-test", "fallback", "load-balance", "relay"];
        if !VALID_GROUP_TYPES.contains(&group_type) {
            errors.push(ValidationError {
                category: "代理组配置".to_string(),
                field: Some(format!("proxy-groups[{}].type", i)),
                message: format!("不支持的代理组类型：{}", group_type),
            });
            continue;
        }

        // 验证 proxies 字段
        match group_obj.get(&serde_yaml_ng::Value::String("proxies".to_string())) {
            Some(proxies) => {
                // proxies 可以是 null（当使用 include-all/filter 时）或数组
                if proxies.is_null() {
                    // proxies: null 是合法的（Clash 会通过 include-all/filter 自动填充）
                    // 不需要验证
                } else if let Some(proxies_array) = proxies.as_sequence() {
                    if proxies_array.is_empty() {
                        errors.push(ValidationError {
                            category: "代理组配置".to_string(),
                            field: Some(format!("proxy-groups[{}].proxies", i)),
                            message: "proxies 数组不能为空".to_string(),
                        });
                    } else {
                        // 检查引用的代理或代理组是否存在
                        for (j, proxy_ref) in proxies_array.iter().enumerate() {
                            if let Some(proxy_name) = proxy_ref.as_str() {
                                // 特殊目标不需要验证（代理组也可以直接使用这些特殊目标）
                                const SPECIAL_TARGETS: &[&str] =
                                    &["DIRECT", "REJECT", "REJECT-DROP", "PASS"];
                                // 引用可以是代理节点或其他代理组或特殊目标
                                if !SPECIAL_TARGETS.contains(&proxy_name)
                                    && !proxy_names.contains(proxy_name)
                                    && !group_names.contains(proxy_name)
                                {
                                    errors.push(ValidationError {
                                        category: "代理组配置".to_string(),
                                        field: Some(format!("proxy-groups[{}].proxies[{}]", i, j)),
                                        message: format!("引用的代理不存在：{}", proxy_name),
                                    });
                                }
                            } else {
                                errors.push(ValidationError {
                                    category: "代理组配置".to_string(),
                                    field: Some(format!("proxy-groups[{}].proxies[{}]", i, j)),
                                    message: "代理引用必须是字符串".to_string(),
                                });
                            }
                        }
                    }
                } else {
                    errors.push(ValidationError {
                        category: "代理组配置".to_string(),
                        field: Some(format!("proxy-groups[{}].proxies", i)),
                        message: "proxies 必须是数组或 null".to_string(),
                    });
                }
            }
            None => {
                // proxies 字段不存在，检查是否有 use 字段（引用 provider）
                if !group_obj.contains_key(&serde_yaml_ng::Value::String("use".to_string())) {
                    errors.push(ValidationError {
                        category: "代理组配置".to_string(),
                        field: Some(format!("proxy-groups[{}]", i)),
                        message: "缺少必需字段：proxies 或 use".to_string(),
                    });
                }
            }
        }
    }

    if errors.is_empty() {
        Ok(group_names)
    } else {
        Err(errors)
    }
}

// 验证 rules 字段（可选）
fn validate_rules(
    root: &serde_yaml_ng::Mapping,
    group_names: &HashSet<String>,
    proxy_names: &HashSet<String>,
) -> Result<(), Vec<ValidationError>> {
    let mut errors = Vec::new();

    // rules 是可选字段
    let rules = match root.get(&serde_yaml_ng::Value::String("rules".to_string())) {
        Some(r) => r,
        None => return Ok(()), // 没有 rules 字段也是合法的
    };

    let rules_array = match rules.as_sequence() {
        Some(a) => a,
        None => {
            errors.push(ValidationError {
                category: "规则配置".to_string(),
                field: Some("rules".to_string()),
                message: "rules 必须是数组".to_string(),
            });
            return Err(errors);
        }
    };

    // 验证每条规则
    for (i, rule) in rules_array.iter().enumerate() {
        let rule_str = match rule.as_str() {
            Some(s) => s,
            None => {
                errors.push(ValidationError {
                    category: "规则配置".to_string(),
                    field: Some(format!("rules[{}]", i)),
                    message: "规则必须是字符串".to_string(),
                });
                continue;
            }
        };

        // 基本规则格式验证：至少包含规则类型和目标
        let parts: Vec<&str> = rule_str.split(',').collect();
        if parts.len() < 2 {
            errors.push(ValidationError {
                category: "规则配置".to_string(),
                field: Some(format!("rules[{}]", i)),
                message: format!("规则格式错误：{}", rule_str),
            });
            continue;
        }

        // 验证规则类型
        let rule_type = parts[0].trim();
        const VALID_RULE_TYPES: &[&str] = &[
            "DOMAIN",
            "DOMAIN-SUFFIX",
            "DOMAIN-KEYWORD",
            "DOMAIN-REGEX",
            "GEOIP",
            "GEOSITE",
            "IP-CIDR",
            "IP-CIDR6",
            "SRC-IP-CIDR",
            "SRC-PORT",
            "DST-PORT",
            "PROCESS-NAME",
            "PROCESS-PATH",
            "RULE-SET",
            "MATCH",
            "AND",
            "OR",
            "NOT",
        ];
        if !VALID_RULE_TYPES.contains(&rule_type) {
            errors.push(ValidationError {
                category: "规则配置".to_string(),
                field: Some(format!("rules[{}]", i)),
                message: format!("不支持的规则类型：{}", rule_type),
            });
            continue;
        }

        // 检查规则目标（代理组或代理）是否存在
        // 规则格式：RULE-TYPE,参数,目标[,选项]
        // 例如：IP-CIDR,1.1.1.1/32,DIRECT,no-resolve
        // 或：MATCH,DIRECT（只有两部分）
        if parts.len() >= 2 {
            // 对于 MATCH 规则，目标在第二部分；对于其他规则，目标在倒数第二或最后一部分
            let target = if rule_type == "MATCH" {
                parts[1].trim()
            } else if parts.len() >= 3 {
                // 如果有 3 个或更多部分，目标可能在倒数第二个位置（如果最后一个是选项如 no-resolve）
                // 或在最后一个位置
                let last_part = parts[parts.len() - 1].trim();
                // 检查最后一部分是否是选项
                const RULE_OPTIONS: &[&str] = &["no-resolve"];
                if RULE_OPTIONS.contains(&last_part) && parts.len() >= 3 {
                    parts[parts.len() - 2].trim() // 目标在倒数第二个
                } else {
                    last_part // 目标在最后
                }
            } else {
                continue; // 格式错误，已在上面报告
            };

            // 特殊目标不需要验证
            const SPECIAL_TARGETS: &[&str] = &["DIRECT", "REJECT", "REJECT-DROP", "PASS"];
            if !SPECIAL_TARGETS.contains(&target)
                && !group_names.contains(target)
                && !proxy_names.contains(target)
            {
                errors.push(ValidationError {
                    category: "规则配置".to_string(),
                    field: Some(format!("rules[{}]", i)),
                    message: format!("规则目标不存在：{}", target),
                });
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

// 检查代理组之间的循环引用
fn check_group_cycles(root: &serde_yaml_ng::Mapping) -> Result<(), Vec<ValidationError>> {
    let mut errors = Vec::new();

    let groups = match root.get(&serde_yaml_ng::Value::String("proxy-groups".to_string())) {
        Some(g) => g,
        None => return Ok(()),
    };

    let groups_array = match groups.as_sequence() {
        Some(a) => a,
        None => return Ok(()),
    };

    // 构建代理组依赖图
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for group in groups_array {
        if let Some(group_obj) = group.as_mapping() {
            let group_name = match group_obj.get(&serde_yaml_ng::Value::String("name".to_string()))
            {
                Some(n) => match n.as_str() {
                    Some(s) => s.to_string(),
                    None => continue,
                },
                None => continue,
            };

            if let Some(proxies) =
                group_obj.get(&serde_yaml_ng::Value::String("proxies".to_string()))
                && let Some(proxies_array) = proxies.as_sequence()
            {
                let mut deps = Vec::new();
                for proxy_ref in proxies_array {
                    if let Some(proxy_name) = proxy_ref.as_str() {
                        // 只记录对其他代理组的依赖（忽略代理节点）
                        deps.push(proxy_name.to_string());
                    }
                }
                graph.insert(group_name, deps);
            }
        }
    }

    // DFS 检测循环
    let mut visited = HashSet::new();
    let mut rec_stack = HashSet::new();

    for node in graph.keys() {
        if !visited.contains(node) && dfs_detect_cycle(node, &graph, &mut visited, &mut rec_stack) {
            errors.push(ValidationError {
                category: "代理组配置".to_string(),
                field: Some(format!("proxy-groups[{}]", node)),
                message: format!("检测到循环引用，涉及代理组：{}", node),
            });
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

// DFS 检测循环引用
fn dfs_detect_cycle(
    node: &str,
    graph: &HashMap<String, Vec<String>>,
    visited: &mut HashSet<String>,
    rec_stack: &mut HashSet<String>,
) -> bool {
    visited.insert(node.to_string());
    rec_stack.insert(node.to_string());

    if let Some(neighbors) = graph.get(node) {
        for neighbor in neighbors {
            // 只检查代理组之间的引用
            if graph.contains_key(neighbor) {
                if !visited.contains(neighbor) {
                    if dfs_detect_cycle(neighbor, graph, visited, rec_stack) {
                        return true;
                    }
                } else if rec_stack.contains(neighbor) {
                    return true;
                }
            }
        }
    }

    rec_stack.remove(node);
    false
}
