#!/bin/sh

systemctl status servus

systemctl stop servus

dpkg -r servus

mv servus_*.deb servus.deb

dpkg -i servus.deb

systemctl daemon-reload

systemctl start servus

systemctl status servus
