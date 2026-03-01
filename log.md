# 第一版
    飞行物做成一个泡泡
    可以被子弹击中得分，时长最短的得分更高。 得分排名存储。
## 泡泡 --完成
## 炮台、子弹 --完成
### 使用leafwing_input_manager改造输出，支持移动端设备 --完成
## 得分 1、需要优化，时间越短，得分越高--完成
## 菜单 --完成
## 排行 --完成，只显示这一局的分数
## 发布 1、调整窗口在不同设备上的适应性 --Over
## 修改移动版本的过关动画，使用magick 生成动图（录制电脑的视频，生成图到手机上播放）
# 第二版
    飞行物做成肥皂泡和各种小动物，有不同的碰撞体积和飞行速度。
    可以被子弹击中得分，时长最短的得分更高。 得分排名存储。

-----安卓编译---------
1、安装android SDK, NDK
2、配置环境变量
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
cargo install cargo-ndk

---android 项目 [armeabi-v7a, arm64-v8a]
cargo clean
cargo ndk -t arm64-v8a build --release --lib
./build-android.bat

---windows项目
cargo clean
cargo build --bin fly_strike
cargo run --bin fly_strike

--最终能在安卓上跑起来---
把bevy里的项目复制过来，在些基础上再修改
主要是改了：1、项目文件名从大写改为小写,再重新生成
修复日志打印：从手机拨号键盘密码打开工程设置界面，打开日志开关

# 生成动图
## 从纯黑背景视频导出png, 
ffmpeg -i 单个烟花.mp4 -vf "fps=30,scale=240:384" frame/frame_%04d.png
//抠透明图--(有问题)
ffmpeg -i 单个烟花.mp4 -vf "fps=30,scale=240:384,format=rgba,colorkey=0x000000:0.03:0.02" frame/frame_%04d.png

ffmpeg -i 三重烟花.mp4 -vf "fps=30,scale=240:384" frame_last/frame_%04d.png

# 合成 SpriteSheet
cd frame
magick montage frame_*.png -tile 11x10 -geometry +0+0 fireworks_sheet.png
cd frame_last
magick montage frame_*.png -tile 11x10 -geometry +0+0 fireworks_last_sheet.png

