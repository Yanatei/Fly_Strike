# 第一版
    飞行物做成一个泡泡
    可以被子弹击中得分，时长最短的得分更高。 得分排名存储。
## 泡泡
## 炮台、子弹
## 得分
## 菜单
## 排行
## 发布

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
cargo ndk -t arm64-v8a -o android/app/src/main/jniLibs build --release
cargo ndk -t armeabi-v7a -o android/app/src/main/jniLibs build --release