FROM cimg/rust:1.65.0-node

WORKDIR .

RUN yarn && yarn deploy