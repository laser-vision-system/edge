BUILD_TARGET=${1:-release}
if [ "${BUILD_TARGET}" = "dev" ]; then 
export BINARY_DIR=debug;
echo $BINARY_DIR;
elif [ "${BUILD_TARGET}" = "release" ]; then 
export BINARY_DIR=release;
echo $BINARY_DIR;
fi