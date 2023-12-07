https://intranet.grid5000.fr/oar/Grenoble/drawgantt-svg/
ssh aoliveiraderosso@access.grid5000.fr
ssh grenoble
oarsub -I
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
git clone https://github.com/ArthurRosso/async_iterator