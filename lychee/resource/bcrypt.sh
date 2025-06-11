#!/bin/bash
 
DEFAULT_COST=10
PASSWORD="$1"
 
echo $PASSWORD | bcrypt -q $DEFAULT_COST
