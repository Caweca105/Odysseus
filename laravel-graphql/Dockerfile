FROM php:8.2-fpm

# Install system dependencies and required PHP extensions
RUN apt-get update && apt-get install -y \
    libpq-dev \
    libzip-dev \
    zip \
    unzip \
    git \
    && docker-php-ext-install pdo pdo_pgsql zip

# Copy Composer from the official Composer image
COPY --from=composer:latest /usr/bin/composer /usr/bin/composer

# Set the working directory
WORKDIR /app

# First copy only composer files to leverage Docker cache
COPY composer.json composer.lock ./

# Create the bootstrap/cache directory which is needed for Laravel
RUN mkdir -p bootstrap/cache

# Install dependencies (without running scripts that might need the full application)
RUN composer install --no-dev --prefer-dist --no-interaction --no-scripts

# Now copy the entire application
COPY . .

# Now run post-install scripts with the full application available
RUN composer dump-autoload --optimize

# Optionally generate the application key (if not set)
RUN php artisan key:generate --force

# Expose the port Laravel will run on
EXPOSE 8002

# Start Laravel's built-in server
CMD ["php", "artisan", "serve", "--host=0.0.0.0", "--port=8002"]
