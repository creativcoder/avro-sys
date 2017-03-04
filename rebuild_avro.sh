
init_build() {
	rm -rf avro/lang/c/build

	mkdir avro/lang/c/build

	cd avro/lang/c/build

	cmake .. -DCMAKE_INSTALL_PREFIX=$PREFIX -DCMAKE_BUILD_TYPE=RelWithDebInfo

	make

	make test

	sudo make install

	cd ../../../..
}

if [ -d "avro" ]; then
	init_build
fi


