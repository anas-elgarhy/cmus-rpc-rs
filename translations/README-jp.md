# ð¦ãæã¤cmusãã¬ã¼ã¤ã¼ã®ããã®Discord Rich Presence

[![crates.io](https://img.shields.io/crates/v/cmus-rpc-rs?style=for-the-badge)](https://crates.io/crates/cmus-rpc-rs)
[![Codacy Badge](https://api.codacy.com/project/badge/Grade/3e0d24aa2c1441e484622b8540193cdf)](https://app.codacy.com/gh/anas-elgarhy/cmus-rpc-rs?utm_source=github.com&utm_medium=referral&utm_content=Anas-Elgarhy/cmus-rpc&utm_campaign=Badge_Grade_Settings)
[![CodeFactor](https://www.codefactor.io/repository/github/anas-elgarhy/cmus-rpc-rs/badge)](https://www.codefactor.io/repository/github/anas-elgarhy/cmus-rpc)

<img alt="image 1" src="../Screenshots/1_0.1.0.png">
<img alt="image 2" src="../Screenshots/2_0.1.0.png">

- å¿è¦ã¨ãã cmus

## ã¤ã³ã¹ãã¼ã«

- ãã crates.io
    ```bash
    cargo install cmus-rpc-rs 
    ```
- ãã Arch User Repository(AUR): `yay -S cmus-rpc-rs`


### ãªãã·ã§ã³:

| ãªãã·ã§ã³                       | èª¬æ                                                  | å¤                                                                                    |
| ---------------------------- | ------------------------------------------------------------ | ----------------------------------------------------------------------------------------- |
| `-h` or `--help`             | ãã«ããè¡¨ç¤º                                                | -                                                                                         |
| `-V` or `--version`          | ãã¼ã¸ã§ã³ãè¡¨ç¤º                                             | -                                                                                         |
| `-d` or `--debug`            | ãããã°ã¢ã¼ã                                                   | -                                                                                         |
| `-l` or `--link`             | cmusã¨ã®é£æºï¼cmusãèµ·åãã¦ããªãå ´åã¯ãã­ã°ã©ã ãéãã¦ãã ããï¼ | -                                                                                         |
| `-c` or `--config`           | ã«ã¹ã¿ã  ãã¹ãæ§æãã¡ã¤ã«ã«è¨­å®ãã                               | è¨­å®ãã¡ã¤ã«ã¸ã®ãã¹ .json                                                                 |
| `-i` or `--interval`         | ãã§ãã¯ééã®è¨­å®                                  | ééæé (ç§)                                                                   |
| `-s` or `--sleep`            | ã¢ã¯ãã£ããã£ããªãã¨ãã«ã¹ãªã¼ããè¨­å®ãã                          | ã¹ãªã¼ãæé (ç§)                                                                      |
| `--p1f` or `--partOneFormat` | æåã®é¨åã®æ¸å¼ãè¨­å®ãã                            | æåã®é¨åã®ãã©ã¼ããã                                                                     |
| `--p2f` or `--partTowFormat` | 2 çªç®ã®é¨åã®æ¸å¼ãè¨­å®ãã                           | ç¬¬ 2 é¨ã®å½¢å¼                                                                    |
| `--li` or `--largeImage`     | ãã¬ã¼ã³ã¹ç¨ã®ã«ã¹ã¿ã å¤§ããªç»åï¼è¡¨ç´ï¼ãè¨­å®ãã                  | å¤§ããªç»åå [ããã©ã«ãã¢ããªã§å©ç¨å¯è½ãªç»å](./assets/cover/)                       |
| `--pi` or `--playingImage`   | ãã¬ã¼ã³ã¹ã®ã«ã¹ã¿ã åçã¤ã¡ã¼ã¸ãè¨­å®ãã                        | åçã¢ã¤ã³ã³å [ããã©ã«ãã¢ããªã§å©ç¨å¯è½ãªãã¬ã¤ç»å](./assets/play_icons/)         |
| `--pai` or `--pausedImage`   | ãã¬ã¼ã³ã¹ç¨ã®ã«ã¹ã¿ã ä¸æåæ­¢ç»åãè¨­å®ãã                         | ä¸æåæ­¢ä¸­ã®ã¢ã¤ã³ã³å[å©ç¨å¯è½ãªã¢ã¤ã³ã³](./assets/pause_icons/)                                 |
| `--pt` or `--playingText`    | ãã¬ã¼ã³ã¹ã®ã«ã¹ã¿ã åçã¢ã¤ã³ã³ ALT ãè¨­å®ãã                     | åçã¢ã¤ã³ã³ã®ä»£æ¿ãã­ã¹ã                                                                     |
| `--pat` or `--pausedText`    | ãã¬ã¼ã³ã¹ã®ã«ã¹ã¿ã ä¸æåæ­¢ã¢ã¤ã³ã³ã®ä»£æ¿ãè¨­å®ãã              | ä¸æåæ­¢ã¢ã¤ã³ã³ã®ä»£æ¿ãã­ã¹ã                                                                      |
| `--b1t` or `--buttonOneText` | ãã¿ã³ 1 ãã­ã¹ã (ã©ãã«) ãè¨­å®                                   | ãã¿ã³ 1 ã¤ã®ã©ãã« (ç©ºã®å ´åãæ§æãã¡ã¤ã«ã«å¤ããªãå ´åããã¿ã³ã¯éè¡¨ç¤ºã«ãªãã¾ã) |
| `--b1u` or `--buttonOneUrl`  | ãã¿ã³ 1 ã® URL ãè¨­å®                                           | ãã¿ã³ 1 ã® URL (æ§æãã¡ã¤ã«ã«å¤ããªããç©ºã®å ´åããã¿ã³ã¯éè¡¨ç¤ºã«ãªãã¾ã)   |
| `--b2t` or `--buttonTwoText` | ãã¿ã³ 2 ã®ãã­ã¹ã (ã©ãã«) ãè¨­å®                                   | ãã¿ã³ 1 ã¤ã®ã©ãã« (ç©ºã®å ´åãæ§æãã¡ã¤ã«ã«å¤ããªãå ´åããã¿ã³ã¯éè¡¨ç¤ºã«ãªãã¾ã) |
| `--b2u` or `--buttonTwoUrl`  | ãã¿ã³ 2 ã® URL ãè¨­å®                                           | ãã¿ã³ 1 ã® URL (æ§æãã¡ã¤ã«ã«å¤ããªããç©ºã®å ´åããã¿ã³ã¯éè¡¨ç¤ºã«ãªãã¾ã)   |

### ä¾:

```bash
cmus-rpc-rs --p1f %title%
```

```bash
cmus-rpc-rs --p1f "%artist% - %title%" --p2f "%album% - %date%"
```

```bash
cmus-rpc-rs --p1f "Anas listening to %title%" --p2f "From %artist%"
```

### cmus èµ·åæã«èªåå®è¡ããæ¹æ³

- æ¬¡ã®è¡ã shellrc ãã¡ã¤ã«ã«è¿½å ãã¾ãã `.bashrc` ã¾ãã¯ `.zshrc`

```bash
    alias cmus = 'cmus-rpc-rs --link &>/dev/null & cmus'
```

### ã§å©ç¨å¯è½

[![GitHub](https://img.shields.io/badge/GitHub-Main%20repo-brightgreen?style=for-the-badge&logo=GitHub)](https://github.com/anas-elgarhy/cmus-rpc-rs)
[![GitLab](https://img.shields.io/badge/GitLab-Mirror%20repo-brightgreen?style=for-the-badge&logo=GitLab)](https://gitlab.com/anas-elgarhy/cmus-rpc-rs)
[![BitBucket](https://img.shields.io/badge/BitBucket-Mirror%20repo-brightgreen?style=for-the-badge&logo=BitBucket)](https://bitbucket.org/anas_elgarhy/cmus-rpc-rs)
[![Codeberg](https://img.shields.io/badge/Codeberg-Mirror%20repo-brightgreen?style=for-the-badge&logo=Codeberg)](https://codeberg.org/anas-elgarhy/cmus-rpc-rs)

### ææ®µ

- [`cmus-remote` tool](https://github.com/cmus/cmus) cmusã«åãåããã
- [Discord Rich Presence](https://github.com/nickofolas/discord-rich-presence) Discord ã® IPC ã¨æ¥ç¶ããããã®ã·ã³ãã«ãªã¯ã­ã¹ãã©ãããã©ã¼ã  ã¯ã¬ã¼ãã§ãã
- [dirs-rs](https://github.com/dirs-dev/dirs-rs) LinuxãmacOSãããã³ Windows ã®ããããã®è¦åã«å¾ã£ã¦ãconfig/cache/data ãã¹ãæä¾ããä½ã¬ãã« ã©ã¤ãã©ãªã
- [clap](https://github.com/clap-rs/clap) Rust ç¨ã®ãã«æ©è½ã®é«éã³ãã³ã ã©ã¤ã³å¼æ°ãã¼ãµã¼ã

[![Quality gate](https://sonarcloud.io/api/project_badges/quality_gate?project=anas-elgarhy_cmus-rpc-rs)](https://sonarcloud.io/summary/new_code?id=Anas-Elgarhy_cmus-rpc)

[![SonarCloud](https://sonarcloud.io/images/project_badges/sonarcloud-black.svg)](https://sonarcloud.io/summary/new_code?id=anas-elgarhy_cmus-rpc-rs)

> ããã¯éãä½¿ã£ãç§ã®æåã®ãã­ã¸ã§ã¯ãã§ã ð, star yoo ã§ç§ããµãã¼ããã¦ãã ãã ðð¦

[![License MIT](https://img.shields.io/badge/license-MIT-green.svg)](https://spdx.org/licenses/MIT.html)
