#!/bin/sh

eval `ssh-agent -s`

ssh-add
ssh-add -l

echo $SSH_AGENT_PID
echo $SSH_AUTH_SOCK

servus