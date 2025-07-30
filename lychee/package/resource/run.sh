
#!/bin/bash
PORT=9808

# 检测端口进程（兼容方案）
PID=$(netstat -nlp 2>/dev/null | awk -v port=":$PORT" '$4 ~ port {split($7,a,"/"); print a[1]}')

if [[ -n $PID ]]; then
    echo "终止进程 $PID (端口 $PORT)"
    sudo kill -9 $PID 2>/dev/null  # 强制终止需权限
    sleep 1  # 等待资源释放
fi

# 启动新进程
echo "Starting cargo program..."
cargo run

