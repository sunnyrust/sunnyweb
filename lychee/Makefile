# cargo build --release
# 定义目标：构建发布版本
build-release:
	cargo build --release
# 复制可执行文件到当前目录
copy-binary:
	cp ../target/release/lychee ./package/
copy-resource:
	cp -rf resource ./package/
copy-configs:
	cp -rf configs ./package/
copy-sunny-derive-trait:
	cp -rf ../sunny-derive-trait ./package/
rm-package:
	if [ -d "package" ]; then rm -rf package/*; fi 
lychee:
#	cd package && ./lychee new && ./lychee db && mv sunny-web ~/work/rust/tests/	
	cd package && ./lychee new --name sunny && ./lychee db --name sunny && mv sunny ~/work/rust/tests/	
create-Readme.md:
	echo "Lychee" > ./package/Readme.md
	echo "A simple and fast web creator written in Rust" >> ./package/Readme.md
	echo "https://github.com/sunnyrust/sunnyweb" >> ./package/Readme.md
	echo "# Command" >> ./package/Readme.md
	echo '```shell' >> ./package/Readme.md
	echo '$$ ./lychee new --name project_name' >> ./package/Readme.md
	echo '$$ ./lychee db --name project_name' >> ./package/Readme.md
	echo '```' >> ./package/Readme.md
	echo "# Create Docker" >> ./package/Readme.md
	echo '```shell' >> ./package/Readme.md
	echo '$$ docker build -t --name project_name .' >> ./package/Readme.md
	# echo '$$ ./lychee db --name project_name' >> ./package/Readme.md
	echo '```' >> ./package/Readme.md
# 定义默认目标
all: rm-package build-release copy-binary copy-resource copy-configs copy-sunny-derive-trait create-Readme.md  lychee

# 清理生成的文件
clean:
	cargo clean

# 运行测试
test:
	cargo test

# 格式化代码
format:
	cargo fmt

# 检查代码
lint:
	cargo clippy
