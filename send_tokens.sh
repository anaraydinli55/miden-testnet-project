#!/bin/bash

addresses=(
  "mtst1aqqd4l85vlzk75txw36lplrxkqws2tkt"
  "mtst1arldk2t7lrs8auf99u3pmrvpl5jvt3t0"
  "mtst1aqnc9cth9rhxvutunrm70wn7qgqrwsyc"
  "mtst1aqghf345kz0qk52vtt9nwy6j7uk6vv2s"
  "mtst1azvx6w63zxgrvugh3qf8zjf00vjgrne6"
  "mtst1az284mq2nqlraugsv09mkplxsg4476d5"
  "mtst1aq24dkpp4mrr4523x6mjlvggv5kcjdt7"
  "mtst1azxa5t9l9smn0u2cyzfku07cpvt2mt4x"
  "mtst1aqf6gcrvmg0gryflhgz2mdt5kquxle7f"
  "mtst1ar8mnqwhxa5075gu3x0kpaf5qyukh252"
  "mtst1azhy54nh5ecsyytat6xjycul0qylrp7t"
  "mtst1apzer3zgq7743v24pj8qe9en7v07yj3s"
  "mtst1aqhmem7qc52evu2mrt2wnj6ahgtgmdrg"
  "mtst1apdnspne8ukqau2jvnee8ya8pg5dxxls"
  "mtst1azf9vqjhzd78jugdm5uajnxltv2m2hfw"
  "mtst1aq2yprfechju6vg9nvzxyf3rlc0z37n4"
  "mtst1az862fwr2peyevgn4vj352xm0vl6zmyh"
  "mtst1arpnx7xakaazsvg2rhmgrzutuv2wqqgw"
  "mtst1aqj93e2yvy5wdv2skadca0vuuypfnp80"
  "mtst1aqajtpcmpqd33u2vgttlud4huuxcwh0j"
  "mtst1azuy0kzje0yp8y2xv9m7j596gypvvwmm"
  "mtst1apcgss9z56wzvy2h7q22twlmu5ggjnyr"
  "mtst1apkjgxfwfawaz5galka373lnxva0vcf2"
  "mtst1apd3szk3c3ckv5t7txwmjrlf3vwxw6yf"
  "mtst1azc5xc3dtn9xvy2g2lr432uwgc266fqa"
  "mtst1azsfhv58muaaqut9ycm5hauafs6u0j8f"
  "mtst1aqkmsuyenav5z5gjukl9gmganvjuy93s"
  "mtst1ar2ux73faech5ygmgjjcnqlylgncmknq"
  "mtst1apd5elsjrhj56u2tfvc2sjr8ygdq2c3l"
  "mtst1arqy2shz4qhfdugye6ah2ekys5gct4rp"
  "mtst1apa7c8vr366g75fk4tda8v2km5hauyc7"
  "mtst1ardcnt0sjmqadygdljff9lrp0ydcut95"
  "mtst1arpw525ms794vugunkt4fh8fmyupy9s6"
  "mtst1ap6ut9mu3jnc45g8p9pvutn7mvuw2rs5"
  "mtst1aq75rw596ezyqu2q5lunayyrrvzsp4fq"
  "mtst1aq90w2wx4p9yyygvsasahewvvy74cdh9"
  "mtst1azmljfrqeecqq5tsps0zenwtssu5xg8a"
  "mtst1apavrde8q72n65tpxh43ql7jzc57c4u2"
  "mtst1aq35wmg4gs9lnu2pcf3d6x09ny6dczm5"
  "mtst1arcxvxhd52sx0yge6kddwxuwlc9mg2vu"
  "mtst1arwcq372rfj7cugllmamv0f3cvr9av3u"
  "mtst1ap9q84lgz9wyyvfwapsyghxew5en57vr"
)

for addr in "${addresses[@]}"; do
  echo "----------------------------------------"
  echo "Gönderiliyor -> $addr"
  # 'echo "y" |' ile gelen onay sorusu otomatik 'y' olarak cevaplanır
  echo "y" | miden client send --target "$addr" --asset 100000000::0xf8b3fd7b01c861715d114ca9c11f78 --note-type public
  sleep 2
done

echo "Tüm transferler başarıyla tamamlandı!"
