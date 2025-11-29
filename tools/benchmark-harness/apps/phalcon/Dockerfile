FROM php:8.2-cli-alpine

# Install essential build tools and dependencies
RUN apk add --no-cache \
    git \
    curl \
    autoconf \
    automake \
    build-base \
    openssl-dev \
    zlib-dev \
    bzip2-dev \
    composer

# Install PHP extensions needed by Phalcon
RUN docker-php-ext-install \
    json \
    mbstring \
    xml

# Download and build Phalcon extension
RUN git clone --depth 1 --branch 5.9.x https://github.com/phalcon/cphalcon.git /tmp/cphalcon && \
    cd /tmp/cphalcon/build && \
    ./install && \
    echo "extension=phalcon.so" > /usr/local/etc/php/conf.d/docker-php-ext-phalcon.ini && \
    rm -rf /tmp/cphalcon

# Create application directory
WORKDIR /app

# Copy composer files
COPY composer.json composer.lock* ./

# Install PHP dependencies
RUN composer install --no-dev --optimize-autoloader

# Copy application files
COPY . .

# Expose port
EXPOSE 8000

# Run server
CMD ["php", "server.php", "8000"]
