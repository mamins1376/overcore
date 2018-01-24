IMAGE_NAME=overcore-buildenv

BUILD_STAMP=build/stamp
DOCKER_FILE=build/Dockerfile
DOCKER_RUN=docker run --mount type=bind,src="${PWD}/src",dst=/build/src,readonly \
		   --rm ${IMAGE_NAME}

all: test

test: ${BUILD_STAMP}
	${DOCKER_RUN} cargo test

build: ${BUILD_STAMP}
	${DOCKER_RUN} cargo build

image: ${BUILD_STAMP}

${BUILD_STAMP}: Cargo.toml ${DOCKER_FILE}
	@echo 'Building build image'
	@sh build/remove-old-image.sh ${IMAGE_NAME}
	docker build -f ${DOCKER_FILE} -t ${IMAGE_NAME} .
	@touch $@

clean:
	@sh build/remove-old-image.sh ${IMAGE_NAME}
	rm -rf ${BUILD_STAMP}
