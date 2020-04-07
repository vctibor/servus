create user servus with encrypted password 'servus';

CREATE DATABASE servus WITH OWNER servus ENCODING = 'UTF8' TEMPLATE template0;

grant all privileges on database servus to servus;