BUILD_TARGET=${1:-release}
if [ "${BUILD_TARGET}" = "dev" ]; then 
echo export BINARY_DIR="debug";
export BINARY_DIR="debug";
elif [ "${BUILD_TARGET}" = "release" ]; then 
echo export BINARY_DIR="release";
export BINARY_DIR="release";