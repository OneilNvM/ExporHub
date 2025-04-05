# Use PHP 8.2
FROM php:8.2-fpm


# Install PHP extension dependencies
RUN apt-get update && apt-get install -y \
    libfreetype-dev \
    libjpeg62-turbo-dev \
    libpng-dev \
    zlib1g-dev \
    libzip-dev \
    unzip \
    && docker-php-ext-install pdo pdo_mysql \
	&& docker-php-ext-configure gd --with-freetype --with-jpeg \
	&& docker-php-ext-install -j$(nproc) gd \
    && docker-php-ext-install zip

# Copy application source and set working directory
COPY . /var/www/app
WORKDIR /var/www/app

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
CMD ["php-fpm"]