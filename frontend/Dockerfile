FROM node:18

# Declare a build arg for the license and pass it as an environment variable
ARG NUXT_UI_PRO_LICENSE
ENV NUXT_UI_PRO_LICENSE=$NUXT_UI_PRO_LICENSE

WORKDIR /app

# Copy only the essential files first to leverage Docker cache
COPY package*.json ./

# Install dependencies (use npm ci if you have a lockfile)
RUN npm install

# Copy the remainder of the source code
COPY . .

# Build the Nuxt application
RUN npm run build

# Expose port 3000 (adjust if your production server runs on another port)
EXPOSE 3000

# Start the Nuxt server (for production, you might consider a production command)
CMD ["npm", "run", "dev"]