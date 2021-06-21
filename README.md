# thes
[![Crates.io](https://img.shields.io/crates/v/thes.svg)](https://crates.io/crates/thes)
[![Crates.io](https://img.shields.io/crates/d/thes)](https://crates.io/crates/thes)
[![API](https://docs.rs/thes/badge.svg)](https://docs.rs/thes)

A Completely Offline Thesaurus CLI Tool (Written in Rust)

## Installation
```
cargo install thes
```

## Usage
Basic usage:
```
$ thes amazing
surprising
astonishing
impressive
awing
awesome
awful
awe-inspiring
```

`thes` can also give us synonyms from data piped in through stdin:
```
$ echo "wonderful" | thes
grand
fantastic
marvelous
extraordinary
wondrous
howling
rattling
tremendous
marvellous
terrific
```

`thes` can print our synonyms horizontally instead of vertically which is the default:
```
$ thes great --horizontal
avid zealous eager enthusiastic smashing not bad peachy slap-up nifty swell good cracking dandy bully groovy corking neat keen bang-up enceinte gravid large pregnant heavy with child expectant big majuscule capital uppercase extraordinary outstanding important of import
```

`thes` can filter our results by their speech with the `type` tag:
```
$ thes good -t noun
commodity
trade good
goodness
```

`thes` can print synonyms with their parts of speech through the `verbose` tag:
```
$ thes cool -v
air-cooled (Adjective)
cold (Adjective)
precooled (Adjective)
air-conditioned (Adjective)
caller (Adjective)
water-cooled (Adjective)
chilly (Adjective)
chill (Adjective)
composed (Adjective)
nerveless (Adjective)
coolheaded (Adjective)
fashionable (Adjective)
stylish (Adjective)
unqualified (Adjective)
unemotional (Adjective)
unagitated (Adjective)
unfriendly (Adjective)
unresponsive (Adjective)
assuredness (Noun)
poise (Noun)
sang-froid (Noun)
aplomb (Noun)
cool down (Verb)
chill (Verb)
cool off (Verb)
```

## Command Line Arguments
```
thes 0.1.3
Grant Handy <grantshandy@gmail.com>
Offline CLI Thesaurus Tool

USAGE:
    thes [FLAGS] [OPTIONS] [WORD]

FLAGS:
    -h, --help          Prints help information
        --horizontal    Prints synonyms horizontally
    -s, --shell         Opens an interactive thesaurus shell
    -V, --version       Prints version information
    -v, --verbose       Prints verbose output, this includes parts of speech for each word

OPTIONS:
    -t, --type <PART OF SPEECH>    Select what parts of speech the synonyms returned will have [possible values: verb,
                                   adjective, adverb, noun]

ARGS:
    <WORD>    Word to find synonyms for
```
