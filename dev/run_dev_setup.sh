#!/bin/bash

export DOXA_BASE_URL=http://localhost:3001

# Working directiory should be in the doxa workspace root
cd "$(dirname "$0")"
cd ..

function create_competition {
  cargo run -q -p doxa_adm -- competition create "$1" --ensure-exists
}

function create_user_invite {
  cargo run -q -p doxa_adm -- invite create -u "$1" --enroll "uttt" --enroll "helloworld"  | tail -n1 | awk '{print $1}'
}

function register_user {
  cargo run -q -p doxa_cli -- register "$1" "$2" -i "$3"
}

function login_user {
  cargo run -q -p doxa_cli -- login "$1" "$2"
}

function make_admin {
  cargo run -p doxa_adm -- user admin promote $1
}

echo 'This script will run the appropriate doxa_adm commands for setting up some example users and
creating the competitions'

echo 'The docker compose dev environment should be running, but the server **should not** be
running'

echo 'press [enter] to continue or [ctrl-c] to exit'
read

cargo build -p doxa_adm
cargo build -p doxa_cli

create_competition uttt
create_competition helloworld

echo 'please start the main server (press [enter] once done & server routes have been setup)'
read

declare -A invites
users=("user1" "user2" "user3" "user4")

for user in "${users[@]}"; do
  echo "Creating invite for $user"
  invite=$(create_user_invite $user)
  invites[${user}]=$invite
done

echo 'Logging in users'

for user in "${users[@]}"; do
  invite=${invites[${user}]}
  echo "Using invite $invite to register & login $user"
  register_user $user password $invite
  login_user $user password
done

echo 'Creating admin user (admin1)'

invite=$(create_user_invite admin1)
register_user admin1 password $invite
login_user admin1 password
make_admin admin1
