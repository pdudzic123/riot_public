# RIOT

Antibodies are a cornerstone of the immune system, playing a pivotal role in identifying and neutralizing infections caused by bacteria, viruses, and other pathogens. Understanding their structure, function, can provide insights into both the body's natural defenses and the principles behind many therapeutic interventions, including vaccines and antibody-based drugs. The analysis and annotation of antibody sequences, including the identification of variable, diversity, joining, and constant genes, as well as the delineation of framework regions and complementarity determining regions, are essential for understanding their structure and function. Currently analyzing large volumes of antibody sequences for is routine in antibody discovery, requiring fast and accurate tools. While there are existing tools designed for the annotation and numbering of antibody sequences, they often have limitations such as being restricted to either nucleotide or amino acid sequences, reliance on non-uniform germline databases, or slow execution times. Here we present Rapid Immunoglobulin Overview Tool (RIOT), a novel open source solution for antibody numbering that addresses these shortcomings. RIOT handles nucleotide and amino acid sequence processing, comes with a free germline database, and is computationally efficient. We hope the tool will facilitate rapid annotation of antibody sequencing outputs for the benefit of understanding of antibody biology and discovering novel therapeutics.


## Requirements
 - Python ^3.10
 - Rust (https://www.rust-lang.org/tools/install)
 - Poetry >= 1.7.1

## Installation

```
poetry install
poetry run maturin develop -r
poetry install     # need to do this once again to be able to use riot executable
```

## Usage
### CLI

```
Usage: riot [OPTIONS]

Options:
  -f, --input-file PATH           Path to input FASTA file.
  -s, --sequence TEXT             Input sequence.
  -o, --output-file PATH          Path to output CSV file. If not specified,
                                  stdout is used.
  --scheme [kabat|chothia|imgt|martin]
                                  Which numbering scheme should be used: IMGT,
                                  KABAT, CHOTHIA, MARTIN. Default IMGT
  --species [human|mouse]         Which species germline sequences should be
                                  used. Default is all species.
  --input-type [nt|aa]            What kind of sequences are provided on
                                  input. Default is nucleotide sequences.
  -p, --ncpu INTEGER              Number of parallel processes to use. Default
                                  is number of physical cores.
  -e, --extend_alignment BOOLEAN  Include unaligned beginning of the query
                                  sequence in numbering.This option impacts
                                  only amino acid sequences passed with -s option.
  --help                          Show this message and exit.
```

#### Examples:
Run on single sequence and print output to stdout:
```
riot -s <sequence>
```
Run on single sequence and save output to csv:
```
riot -s <sequence> -o result.csv
```
Run on fasta file:
```
riot -f input.fasta -o results.csv
```
### API Nucleotides

```
from riot import create_riot_nt, Organism, Scheme, RiotNumberingNT, AirrRearrangementEntryNT

riot_nt: RiotNumberingNT = create_riot_nt(allowed_species = [Organism.HOMO_SAPIENS])
airr_result: AirrRearrangementEntryNT = riot_nt.run_on_sequence(
                    header = "SRR13857054.957936",
                    query_sequence = "GAACCAAACTGACTGTCCTAGGCCAGCCCAAGTCTTCGCCATCAGTCACCCTGTTTCCACCTTCCCCTGAAGAGCTAAAAAAA",
                    scheme = Scheme.KABAT
                )
```

### API Amino Acids

```
from riot import create_riot_aa, Organism, Scheme, RiotNumberingAA, AirrRearrangementEntryAA

riot_aa: RiotNumberingAA = create_riot_aa(allowed_species = [Organism.HOMO_SAPIENS])
airr_result: AirrRearrangementEntryAA = riot_aa.run_on_sequence(
                    header = "SRR13385915.5101835",
                    query_sequence = "QVTLKESGPVLVKPTETLTLTCTVSGFSLSNARMGVSWIRQPPGKALEWLAHIFSNDEKSYSTSLKSRLTISKDTSKSQVVLTMTNMDPGDTATYYCARRGGTIFGVVIILVRRPPL",
                    scheme = Scheme.KABAT,
                    extend_alignment = True
                )
```