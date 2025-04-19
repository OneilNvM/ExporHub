# Use PHP 8.4-fpm
FROM php:8.4-fpm-alpine3.21

# Install php-extension-installer 
# COPY --from=mlocati/php-extension-installer /usr/bin/install-php-extensions /usr/local/bin/

# Configure zz-docker.conf file for php-fpm.d
RUN echo 'pm.max_children = 12' >> /usr/local/etc/php-fpm.d/zz-docker.conf && \
    echo 'pm.start_servers = 6' >> /usr/local/etc/php-fpm.d/zz-docker.conf && \
    echo 'pm.min_spare_servers = 2' >> /usr/local/etc/php-fpm.d/zz-docker.conf && \
    echo 'pm.max_spare_servers = 12' >> /usr/local/etc/php-fpm.d/zz-docker.conf && \
    echo 'pm.max_requests = 4000' >> /usr/local/etc/php-fpm.d/zz-docker.conf

# Install PHP extension dependencies
RUN apk update && apk upgrade && apk add --no-cache libjpeg-turbo-dev \
    freetype-dev \
    libpng-dev \
    libzip-dev \
    unzip \
    && docker-php-ext-install pdo pdo_mysql \
    && docker-php-ext-configure gd --with-freetype --with-jpeg \
    && docker-php-ext-install -j$(nproc) gd \
    && docker-php-ext-install zip \
    && docker-php-ext-install opcache \
    && docker-php-ext-install pcntl
# && install-php-extensions xdebug

COPY ./conf.d/opcache.ini /usr/local/etc/php/conf.d/opcache.ini

# Copy application source and set working directory
COPY . /var/www/app
WORKDIR /var/www/app

RUN mkdir -p /etc/nginx/ssl

# COPY ./xdebug/xdebug.ini /usr/local/etc/php/conf.d
COPY ./nginx/nginx.conf /etc/nginx

RUN --mount=type=bind,source=certificates,target=/etc/nginx/ssl

# Set folder owner and change permissions to read and write
RUN chown www-data:www-data /var/www/app \
    && chmod -R 775 /var/www/app/storage

# Install composer
COPY --from=composer:2.6.5 /usr/bin/composer /usr/local/bin/composer

# Copy composer.json to working directory and run composer
COPY composer.json ./
RUN composer install

# Set the default command to run php-fpm
ENTRYPOINT ["php", "artisan", "octane:frankenphp"]