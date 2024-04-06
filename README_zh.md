# Maa Resonance

[简体中文](/README_zh.md) | [English](/README.md)

这是一个用tauri创建的Maa Resonance前端项目。

## 项目目标 （大饼）

- 自动跑商
  - 根据不同策略消耗疲劳度来选择不同路线
- 清理澄明度

## 快速开始

请注意当前版本构建只在 Windows 测试过。 对 Linux 系统的支持在进行中。

### MaaAgentBinary

这个仓库包含了  `MaaAgentBinary` 作为子模块。 你需要用该参数  `--recurse-submodules` 来 clone 该仓库或者在 clone 后运行如下指令:

```bash
git submodule init  
git submodule update
```

### MaaFramework

你需要安装最新版本的 MaaFramework（v1.7.*）。如果已经安装，请确保 CMake能够找到它。最简单的方法是将 MaaFramework 的安装目录添加到 `CMAKE_PREFIX_PATH` 或 `PATH` 环境变量中。

如果你没有安装，python 脚本将会安装，没有手动安装的必要。

### 运行依赖脚本

如果你已经拥有了安装后的 MaaFramework，你可以跳过该步骤并且手动复制 MaaFramework 的所有 dll 到 `tauri/` 目录下

否则可以运行如下指令

```bash
python ./scripts/makedeps.py
```

 来下载并提取最新版本的 MaaFramework 到 `tauri/` 目录下。然后，如上一步所说，你需要添加环境变量指向脚本安装的 MaaFramework 目录，即 `$projectDir/scripts/deps/maafw`。

CMake 和 clang (你可以用LLVM来替代) 同样需要安装。

### 构建并运行

推荐使用 `pnpm` ，也可以使用 `npm` 或者 `yarn`

```bash
pnpm install  
pnpm tauri dev
```

我们还提供 `mock` 命令，用于在扫描时显示一个模拟设备，这对UI开发非常有用。

```bash
pnpm mock
```

然后用如下指令构建项目：

```bash
pnpm tauri build
```
