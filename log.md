# 第一版
    飞行物做成一个泡泡
    可以被子弹击中得分，时长最短的得分更高。 得分排名存储。
## 泡泡 --完成
## 炮台、子弹 --完成
## 得分 1、需要优化，时间越短，得分越高
## 菜单 --完成
## 排行
## 发布 1、调整窗口在不同设备上的适应性

# 第二版
    飞行物做成肥皂泡和各种小动物，有不同的碰撞体积和飞行速度。
    可以被子弹击中得分，时长最短的得分更高。 得分排名存储。

-----安卓编译---------
1、安装android SDK, NDK
2、配置环境变量
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
cargo install cargo-ndk


--windows编译----
cargo run --features desktop

---android 项目 [armeabi-v7a, arm64-v8a]
cargo clean
cargo ndk -t arm64-v8a build --release
cargo ndk -t arm64-v8a -o android/app/src/main/jniLibs build --release
cargo ndk -t armeabi-v7a -o android/app/src/main/jniLibs build --release

--这二个只能存在一个
[lib]
[bin]

--最终能在安卓上跑起来---
把bevy里的项目复制过来，在些基础上再修改
主要是改了：1、项目文件名从大写改为小写,再重新生成
修复日志打印：从手机拨号键盘密码打开工程设置界面，打开日志开关