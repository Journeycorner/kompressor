FROM fholzer/nginx-brotli

COPY target/deploy /usr/share/nginx/html
COPY server/nginx.conf /etc/nginx/nginx.conf
COPY server/server.crt /etc/ssl/server.crt
COPY server/server.key /etc/ssl/server.key