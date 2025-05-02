# bevy_mod_eninge
a mulitype thread mod enigne for bevy and other rust based game enignes



https://www.swift.org/install/linux/swiftly/



cargo install cbindgen


mkdir setup
cd setup
curl -O https://download.swift.org/swiftly/linux/swiftly-1.0.0-$(uname -m).tar.gz
curl https://www.swift.org/keys/all-keys.asc | gpg --import -
curl -O https://download.swift.org/swiftly/linux/swiftly-1.0.0-$(uname -m).tar.gz.sig
gpg --verify swiftly-1.0.0-$(uname -m).tar.gz.sig swiftly-1.0.0-$(uname -m).tar.gz

# note . "/home/codespace/.local/share/swiftly/env.sh" add something simlar 
