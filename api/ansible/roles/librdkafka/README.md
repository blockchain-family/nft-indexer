librdkafka
=========

[![Build Status](https://travis-ci.com/rgsystemes/ansible-role-librdkafka.svg?branch=master)](https://travis-ci.com/rgsystemes/ansible-role-librdkafka)

Installs [edenhill/librdkafka](https://github.com/edenhill/librdkafka) from sources on **Debian/Ubuntu**. 

Tested with :
- Debian 10.x :heavy_check_mark:
- Debian 9.x :heavy_check_mark:
- Ubuntu 20.04.x :heavy_check_mark:
- Ubuntu 18.04.x :heavy_check_mark:
- Ubuntu 16.04.x :heavy_check_mark:

Role Variables
--------------

```yaml
---
librdkafka_cleanup_if_exists: false # Turn to true if you wish to overwrite existing assets (weither it comes from source or apt). Usefull for upgrade/downgrade.
librdkafka_version: v1.4.2 # the release number to get from GitHub. See: https://github.com/edenhill/librdkafka/releases 
# librdkafka_checksum: # specify matching SHA256's checksum if you want to
# librdkafka_build_opts: # See: https://github.com/edenhill/librdkafka#building 

```

Example requirements file
------------------------

```yaml
---
- src: https://github.com/rgsystemes/ansible-role-librdkafka
  name: rgsystem.librdkafka
  version: master

```

Example playbook
------------------------

```yaml
---
- hosts: servers
  become: yes
  gather_facts: yes

  roles:
    - name: rgsystem.librdkafka
      vars:
        librdkafka_cleanup_if_exists: true
        librdkafka_version: v1.4.0
        librdkafka_checksum: sha256:ae27ea3f3d0d32d29004e7f709efbba2666c5383a107cc45b3a1949486b2eb84
        librdkafka_build_opts: --install-deps

```

Dependencies
------------

None

License
-------

MIT / BSD
