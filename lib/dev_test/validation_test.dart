import 'dart:async';
import 'dart:io';
import 'package:stelliberty/utils/logger.dart';
import 'package:stelliberty/src/bindings/signals/signals.dart';

// 配置验证测试
//
// 测试 Rust 层的 Clash 配置验证功能
// 批量验证 assets/test/config_validation_tests 目录下的所有 YAML 文件
class ValidationTest {
  // 测试目录路径
  static const testDir =
      'E:\\All Codes\\Flutter\\stelliberty\\assets\\test\\config_validation_tests';

  // 运行验证测试流程
  static Future<void> run() async {
    Logger.info('========================================');
    Logger.info('配置验证测试启动');
    Logger.info('========================================');

    try {
      // 扫描测试目录
      final dir = Directory(testDir);
      if (!await dir.exists()) {
        Logger.error('测试目录不存在: $testDir');
        Logger.info('请创建该目录并放置测试 YAML 文件');
        exit(1);
      }

      // 获取所有 .yaml 和 .yml 文件
      final files = await dir
          .list()
          .where(
            (entity) =>
                entity is File &&
                (entity.path.endsWith('.yaml') || entity.path.endsWith('.yml')),
          )
          .cast<File>()
          .toList();

      if (files.isEmpty) {
        Logger.warning('测试目录中没有找到 YAML 文件: $testDir');
        Logger.info('请在该目录下放置 .yaml 或 .yml 文件');
        exit(0);
      }

      Logger.info('📂 找到 ${files.length} 个测试文件');
      Logger.info('');

      // 统计结果
      int passedCount = 0;
      int failedCount = 0;
      final failedFiles = <String>[];

      // 逐个验证文件
      for (int i = 0; i < files.length; i++) {
        final file = files[i];
        final fileName = file.path.split('\\').last;

        Logger.info('----------------------------------------');
        Logger.info('[${i + 1}/${files.length}] 验证文件: $fileName');
        Logger.info('----------------------------------------');

        try {
          final content = await file.readAsString();
          Logger.info('文件大小: ${content.length} 字节');

          // 调用 Rust 验证
          final result = await _validateWithRust(content, fileName);

          if (result) {
            Logger.info('✅ $fileName 验证通过');
            passedCount++;
          } else {
            Logger.error('❌ $fileName 验证失败');
            failedCount++;
            failedFiles.add(fileName);
          }
        } catch (e) {
          Logger.error('❌ $fileName 验证异常: $e');
          failedCount++;
          failedFiles.add(fileName);
        }

        Logger.info('');
      }

      // 输出测试总结
      Logger.info('========================================');
      Logger.info('测试总结');
      Logger.info('========================================');
      Logger.info('总文件数: ${files.length}');
      Logger.info('通过: $passedCount');
      Logger.info('失败: $failedCount');

      if (failedFiles.isNotEmpty) {
        Logger.info('');
        Logger.error('失败的文件列表:');
        for (final fileName in failedFiles) {
          Logger.error('  - $fileName');
        }
      }

      Logger.info('========================================');

      // 如果有失败的测试，退出码为 1
      if (failedCount > 0) {
        exit(1);
      }
    } catch (e, stackTrace) {
      Logger.error('========================================');
      Logger.error('验证测试异常: $e');
      Logger.error('堆栈跟踪: $stackTrace');
      Logger.error('========================================');
      exit(1);
    }
  }

  // 使用 Rust 验证配置
  static Future<bool> _validateWithRust(String content, String fileName) async {
    final completer = Completer<bool>();

    // 订阅 Rust 信号流
    final streamListener = ValidateSubscriptionResponse.rustSignalStream.listen(
      (result) {
        if (!completer.isCompleted) {
          if (result.message.isValid) {
            Logger.debug('Rust 返回: 验证通过');
            completer.complete(true);
          } else {
            Logger.error(
              'Rust 返回: 验证失败 - ${result.message.errorMessage ?? "未知错误"}',
            );
            completer.complete(false);
          }
        }
      },
    );

    try {
      // 发送验证请求到 Rust
      final request = ValidateSubscriptionRequest(content: content);
      request.sendSignalToRust();
      Logger.debug('验证请求已发送到 Rust');

      // 等待验证结果（30 秒超时）
      final result = await completer.future.timeout(
        const Duration(seconds: 30),
        onTimeout: () {
          Logger.error('验证超时（30秒）');
          return false;
        },
      );

      return result;
    } finally {
      // 停止监听信号流
      await streamListener.cancel();
    }
  }
}
