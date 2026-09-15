#!/bin/bash

addresses=(
  "mtst1arf6kk4x8ngwdgf2cfer5y0jwy87p7yr"
  "mtst1aqx8h4nr2zdsayfvk2u02avlmq7jehdl"
  "mtst1az3g0qxkzl63a52vzda9azdwq5e2mdy3"
  "mtst1aqln8c66nmnxwy2qwg9nej8nrujxlh4c"
  "mtst1apzer3zgq7743v24pj8qe9en7v07yj3s"
  "mtst1apgw5errnpcrqvg3wgwmtkt2vvmrkn3s"
  "mtst1azwddxmqw43xyyfchu93sj54ksg2d5zr"
  "mtst1aqt64yrkd46khqfe32dlp02gtcpeglvy"
  "mtst1aq459l3kvvaawsfxpmxqxszd35qxkgn9"
  "mtst1arq42nuc5cp3352s6pry0k3lqq3kwu68"
  "mtst1artza6fag82s5yt8kvxlzygwvsl96zxx"
  "mtst1aq7jk487x6hn4sgrqjfeu9usl5tx6t2q"
  "mtst1apsdmlrdz6fyzu29l4uklv3s7gnq6sjp"
  "mtst1aqlvrt4ay06j852sne0886lrkg8eg768"
  "mtst1aqnem5hxu5k9zyfqdllaydqa3gvc5asw"
  "mtst1az8qd6u7glwd852vwj6lasx34shmck93"
  "mtst1ap3u0y9nk877yy2wqjfnuh0gtg4ekwmf"
  "mtst1ar7dm7j9d8qqayfe8tslsp9xystwvetu"
  "mtst1ar7asz4jxu6j4yt62l9khvrcyqp389sl"
  "mtst1ap9uynm7kv3xgq2gjpgm59hs4qlvgeuu"
  "mtst1aq6676qs804jwv2t6e44hqxj2gkac7u3"
  "mtst1ap0xqtz8vfg6vvttp7ap7v86gcufan3z"
  "mtst1arutys2647pskcf0v96qghnv3u36hk7t"
  "mtst1arec3t23c0culvtdjzkqxejc7vvddf3u"
  "mtst1aqry8n7ad8n78vftr7p0uwa8pqgudjpx"
  "mtst1azsns7498gkxru2z2nyvfc7u0uxwrulk"
  "mtst1az9fx3ga3sa8vvt0fne4em4suux7klc2"
  "mtst1aqk35uzpwxxxmvtc8jlx2cyv0ckkkyxv"
  "mtst1azj8r6tlnn4fa5f7a627td4mvykvu0xr"
  "mtst1azj24q0qm2e7kut5k0puux8egszuxq6h"
  "mtst1arnjwm47n9pk052yjmhd7nqpesajl950"
  "mtst1azf3kxe75whvq5gwrdh4t2q7mqchaae8"
  "mtst1arzh6pdava0khuft6js9x5kx9vwqntkh"
  "mtst1aqe2wuf5n59alyghk6vpln4lq53c5t0m"
  "mtst1ar2pakhtttcssu20mmmyra5csu3rwzgs"
  "mtst1azx4px6x562hru26n376f7gzxsqj83a7"
  "mtst1azcv6g8tjypehutfqpqzjg2yt5p8ryey"
  "mtst1aqd57gwd4c4p452mjc4qj6h4zqsj04vv"
  "mtst1azs3zsftz4y5wyg9ds0fr673cyzwdqhs"
  "mtst1aqqqhkj58yzt9ug64xdylyrpv5jm25zj"
  "mtst1arz7nm8zldjzmuf0zjw55ehs0qzdpqta"
  "mtst1aq9n68xuknk29uflsvx9455fyupjklc8"
  "mtst1azj4mzc8ndr3rufx0m99hwnl2gw8qnyl"
  "mtst1arfpmu85palspytwm4wdvz92857s3ruv"
  "mtst1ap2c0ywp5yn4aut7a79u59str5d68vt5"
  "mtst1aqs5jrfkk4ejlyfgqgpkyq2aguu8zkff"
  "mtst1aqkke3zdumeu4ufwgd336v0g3y9xp65x"
  "mtst1aptpmpca6clmdufssuvn5rrrhsl3ly89"
  "mtst1aqj93e2yvy5wdv2skadca0vuuypfnp80"
  "mtst1arygqzyfp40flvfs42kgsyp3r5wl9r8e"
  "mtst1aq66nryfdkfsl5g4ylv7h5547uv0n2tu"
  "mtst1azmrpz86xm6qayf66guqpdcchvptqmuz"
  "mtst1azqc54c2mc7kjvty4hgzruhatsxm3dh8"
  "mtst1ar0fwlcdceawa5g5zyfp623ku5racwm0"
  "mtst1azhw4h0rf7l8552n4cfa85jv9sx9xmvg"
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
