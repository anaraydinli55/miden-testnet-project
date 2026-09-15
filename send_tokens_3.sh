#!/bin/bash

addresses=(
  "mtst1apge2pehn43dyv2khn94gp4ns5j2drr8"
  "mtst1ar4gacgxkdx0z52ypt4v6fr8hvplvj30"
  "mtst1aqd7av6sge98wu2da7c2p6q6qyl8pm08"
  "mtst1argk04pry2gfjvtw4hvkkvdh4ytlmsrt"
  "mtst1apw5mkh835zyhyf40y5mn6z2hvt08m5s"
  "mtst1azlhdalh7lc90u29y2l8yw83avpncwj4"
  "mtst1azwj4e9l9f3qvv2pt3hvng525qxmcs08"
  "mtst1apzer3zgq7743v24pj8qe9en7v07yj3s"
  "mtst1aqs53gzpmrj3xvt3w9xs5p88eqwp7d0t"
  "mtst1az8cdapyd3p2r5ttxp9ycc7aj5nwjhma"
  "mtst1ar46f3ykr77hqvtltxw0c3nw2sy0hew2"
  "mtst1aq5zztffpk8nsytt50p8ju5hyqkszamm"
  "mtst1az9jyv0rrq9z5yf5d4573jcals3rqxqv"
  "mtst1azxvzms3uuu6ay2n0h8vj2up8sapw96t"
  "mtst1azmurnxd6ycp5yg3qra5ugvc35hd826s"
  "mtst1azxu5sr3fx9v2yf70sx3uk2d6uwe8x5z"
  "mtst1ary6g7gdthj8sug2qknpp9q7rvsa2452"
  "mtst1ap53pnq4j55wjufgnlt0j694hc6cq7a8"
  "mtst1azukjl6ph9a9pyg29wcl72fgaqny2z9d"
  "mtst1artf9ggme5waqytuqsa43d99gyxndsl5"
  "mtst1aptkqc2zwnru45fdzznn3f92fv5kp4gz"
  "mtst1azl95xhkvu6c8vg94zujluru0gem282u"
  "mtst1aq5jzzc54kmsjygmqnvpy8rk056dh233"
  "mtst1arkdqlm5f5lkrvt39c56hty9kyk5lrrt"
  "mtst1apnnlydqaz2mjuguqgvc0z7rxuw54mdz"
  "mtst1apu2mvtcqwrwfsgg2me0gy3lts2sxqg4"
  "mtst1ap4sk72wwvepj52g6t06ljgy9q5d4zgn"
  "mtst1aqk0ea5yakwyw5ggdqqxp2cf0sm0n9y8"
  "mtst1ar5ta79phn3z5yfk0w5t3g3yrqg9c9ff"
  "mtst1arnw0dxqw5xdeyfa8m5gmlsuy5fr7y3h"
  "mtst1arv4c07t9659zyte2fxrrpl75vtc053s"
  "mtst1aru3wx8jpw255vfg5kllsq2jvvwe6mdx"
  "mtst1aqggcz4hkx9wdugvdvp3aj33wux7ty78"
  "mtst1ar2rh6572hg5c5tvsye5eqp4my5z6qnh"
  "mtst1az58e7qphhfrhv2lps6kd0rw5cpq5g40"
  "mtst1arjuy75yp20mjvte5w9asuyyase6e5pf"
  "mtst1ar0yccjp82tw8y2g8zujasz9ay5ly20h"
  "mtst1aquh3xd404hu5vg3dme73khekyvrasaq"
  "mtst1aq3vqhqsh64fqv2fy0lsytc5hsmuejf8"
  "mtst1azk2mmn9dlt4evf750fywk58gvfya0a6"
  "mtst1aqjphx6krg0zaqtf78hrcfmktuxx7mxg"
  "mtst1aqlxmr830qjaf5gek4sevr0chcryjkve"
  "mtst1aqnv787x0mxue5ffm2mnaf4s2qhxewhf"
  "mtst1aprveym222p89sfraeuk7w29qvtyv8h6"
  "mtst1aqz3wg0na27u95ffcktwncg2zu48j5zv"
)

total=${#addresses[@]}
count=1

for addr in "${addresses[@]}"; do
  echo "========================================"
  echo "[$count / $total] Gönderiliyor -> $addr"
  echo "========================================"
  echo "y" | miden client send --target "$addr" --asset 100000000::0xf8b3fd7b01c861715d114ca9c11f78 --note-type public
  ((count++))
  sleep 2
done

echo "Tüm liste için gönderimler başarıyla tamamlandı!"
