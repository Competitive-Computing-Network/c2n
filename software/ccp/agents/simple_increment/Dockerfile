# Use an official Nix image as a base image
FROM nixos/nix:2.21.4

# Enable the experimental Nix features
RUN mkdir -p /etc/nix && \
    echo "experimental-features = nix-command flakes" >> /etc/nix/nix.conf

# Install necessary dependencies and set up the environment
RUN nix-channel --update && \
    nix-env -iA nixpkgs.cacert

# Copy the entire project directory into the container
COPY . /usr/src/simple_increment

# Set the working directory in the container
WORKDIR /usr/src/simple_increment

# Set the entrypoint to nix develop for interactive use
ENTRYPOINT ["nix", "develop", "--impure"]
