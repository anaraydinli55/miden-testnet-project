#!/bin/bash

cd ~/midenup/miden-testnet-project

# Varsayılan hesabın ana cüzdanın olduğundan əmin ol
miden-client account --default 0x86341565987080016e478676445d61

# Sənin cüzdanın xaric edilmiş 101 aktiv testnet ünvanı
TARGETS=(
  "mtst1aq32gfucapgeey2zznc6vvqfeqh5h4rt"
  "mtst1arpcmvvf9y99r5gd34pyynynnynx2790"
  "mtst1aqvpq8a9ytqhfvt9al20wzsrs56g83ec"
  "mtst1azm83hvhskp9jy2fk6k67q8gf5gzhjkh"
  "mtst1apnc56sdll470yg6ewujua9nncsxpncv"
  "mtst1arwuv3gqmvtsu5gmuxyhc9hyugn2m0dz"
  "mtst1aqakdcst2zy0y5fnk6fpvjzx2g8tqn65"
  "mtst1aqpj69hppmzzcs2f0psmcdevwcck5qj5"
  "mtst1azm64e0d23cguqf7dxm7we8hgu65wu25"
  "mtst1apt8ea6x3vhddsf8q8wlrnpe2yru3v2m"
  "mtst1azddte04f8q055ttdz8twppanyekrl2e"
  "mtst1aqznzkxu735aeqfvclnkhplvwya2teux"
  "mtst1arq0wylqjv09eqfwfglljy7shgv8dtyx"
  "mtst1azmzve9f30xmrqgexw2hh3phnyka3l8k"
  "mtst1aznyl6hgy87n452sm0p5nscwxu7vra94"
  "mtst1apu9jlmj26xndstn6fjdplvqjqj653a0"
  "mtst1azmtjkr573atkqgd2mc6acsvyyypqpkr"
  "mtst1arwzlnwkuk8z4sf302t2wsu9kukcdxxa"
  "mtst1azca3svqjavnaq2kjxht9l8y4vrzjx3j"
  "mtst1arj73z7u8h993q2sq7ursc3y6gvh27ja"
  "mtst1apkmwv2y0mfs85gf0v2wnhh60gfm5lrf"
  "mtst1az3rcn8z5gjg7stmruqqw07v2cn6t7pr"
  "mtst1aqgmdalw5s3kgsgannyr24ft8cfyr2hg"
  "mtst1azdmrtkzjhfyzs24vyws39enqgrkqc4x"
  "mtst1aqkpvyfxlptsvqf5v7g74xu2fyfxmlj9"
  "mtst1azs3gxlsdupf05gan5c4sj5xu5f6q9qx"
  "mtst1apvpadhyl3q925t4cnm5q2lufgunc6qe"
  "mtst1azlhrru8x3pl4sg32nz0g7dcgcnjxkln"
  "mtst1ar5v8zyzaenuq5tptcl5ax74jv7d66qn"
  "mtst1apsrpk5v3umukq26fvakxsgq0gkv62sp"
  "mtst1azx58dypxzharyftxwdss0evdqj95wyn"
  "mtst1aphz86n80ucplsf90zhtlhxtg5q3r3cu"
  "mtst1aqewpkgl9sl6rq25g0qmcwr5jqhnht49"
  "mtst1aqw6xdwdn86675fk8affxqmdwue4xjth"
  "mtst1aq76hzrrm4u86yt2yvs68gkxeg9v8pgy"
  "mtst1apu29fpd549xryg327xwnu8tvv3uj4zf"
  "mtst1apuq65vzj9r6p52wm6fw7kq4zg6gqkng"
  "mtst1apjtld2s6puzzy2fypc3amj3avqgx7wq"
  "mtst1aq8pf06dykj4yyfeh9ffqff6sg5qrt90"
  "mtst1apfgwadjm5qdu5gdvtly3ngkq5vgl7cw"
  "mtst1ar0hnf3p8ulpwyg7phdqrgvgx5qk6lkd"
  "mtst1aqewhxltwv0z9yf3g00p0klyncgw255u"
  "mtst1apuar7tvx9m2py2cqq8e8yrfdvuy9t88"
  "mtst1ap0zvrs6gckyfq24vjml0ghlngyp6m3u"
  "mtst1ap3r77n56f3fpyt7rshnn4a7jvr40x3g"
  "mtst1arvul2ypva4lestzc4xuz6m2cyjglhe6"
  "mtst1apfrrge9ctzetsft2wprdzg35ggkm3m7"
  "mtst1az0nk5qqmfe93stdzkgdyc7ytshm4l52"
  "mtst1arqxg9er3xclayt95nud82jnpggl9azj"
  "mtst1aqcl5scrfe8trsfeq79dud54tuq83vns"
  "mtst1azhhmclv62vlnsfzhdh20cyynqq8ec7n"
  "mtst1ars7s3dna74yu52u0n9hrh3muunn2w2t"
  "mtst1ar2uw73wtd80gsglargx0zag95hq0vum"
  "mtst1apv5y3ajmdh6tqtmsvygvm3ljg72p6lw"
  "mtst1ap042uw0vrk76sf7d6djeuf83s00xu8t"
  "mtst1azdny2n90jf7p5t6fegjlvd4xg0uh8d6"
  "mtst1armgmdpfmw80csfvy949x3t43q7g3u3t"
  "mtst1aztrrf72lw2h5qtksdyppf4nrc6ryz8f"
  "mtst1apwm56xzv3dauq2magd4m5dk6umm9203"
  "mtst1azpp2ystz075sqtt3kur9xyryshgk8cr"
  "mtst1aq268mnwrx7v4sfjqgsr73mlduud7c2h"
  "mtst1apxywcjdudcz9stpeftrp47l0yz4fta0"
  "mtst1aqv6lxhhr47qxsgdxmzj8r0hkvclqs80"
  "mtst1ar6je0ccyts3hsts836v9ft2nuszwa2c"
  "mtst1aqklcpsfgce2fqtl7stauhsc95wp0hpp"
  "mtst1apqhzp0gv4nysyft9hylzuqzk5l5yp70"
  "mtst1ar6zjd2rwn6m4qfck9ksm6cwzqaxld8t"
  "mtst1apuxn62z9jllystkjtagf7xsqqzt6q6t"
  "mtst1arj5qh8tc6rpnstpu35gxle0gghuqkql"
  "mtst1arnrvg6f7ten3sfv8j3pr0gde5s9ychm"
  "mtst1arz83e8z86u93s29u75ezx670s2jmwlp"
  "mtst1apfv7epfwvyx6sg27nwy99p6hy9zlxf9"
  "mtst1arpn3zdddlwyws2qjun0zsm0pvtwd50n"
  "mtst1aqlzglvwyclymqg453fn8uycr5skhjah"
  "mtst1arsp3n6y99ngastjmgj0t6v0cc8fzukz"
  "mtst1aqzvkfcpjrzaxsfy77enllyt55szv53g"
  "mtst1aztcmft8l57kdstk9cu448vncu23dk52"
  "mtst1ap44n2ptt485ksgepkgqj63d6cq0hrhp"
  "mtst1aqqndg7fd4wj8s22j9hfmf79rqjmj5qp"
  "mtst1ap7j6grd0hkqmqtt94dtzvz8mc6r0ln6"
  "mtst1az6dl2hr8egs5q2kg60f97ee3gaahft0"
  "mtst1aq64vuqjv8xn5sfqj6ggayg2z5x7thhk"
  "mtst1arpdzta6p0l2ey2aq33fmf3gmv4q0fs8"
  "mtst1aqq58wkw5dqhvsfpy7juyrj9vqgurdjz"
  "mtst1arm38azlp0e0rq262syra42uzvrmm9v6"
  "mtst1arjt44pn240vxqg34u78r37evcd4wx0g"
  "mtst1ap4jpdmrthc9fq2f8xzrwvnl9u3kktnl"
  "mtst1aq30jv2m2zy6ysfhayuvgmj46cn08r78"
  "mtst1az0t9fsxcz2ufsgu4hnn5eh0cuu9g6n9"
  "mtst1aqhyf2fct9gxhsf720pzzsv2uspzpwev"
  "mtst1aq7xw6gdxjwpusfn5mlf85ut3qfe2m7s"
  "mtst1ap74gr423tdxhqtpp4vu985j7c5mjhy7"
  "mtst1azdj858rx732rsgav6d080s7gs2y8quq"
  "mtst1apy5j23nkrhe5q2kpnzxrtd3kqr7kd3z"
  "mtst1azpx5gnkcn6ugqgw3n4w3g44p57eax75"
  "mtst1aqjkcpasxc6h4q2yx2xp989jtgpgd2p9"
  "mtst1aq9nwte8xh3nayfpdkv4hu5mjy90arem"
  "mtst1arn2qpkvmaa3p5f0z40jz4magv2rvju4"
  "mtst1aqqrskmeh8as4qfcl3024q3flv3n4jke"
  "mtst1azdec86z0k3s7s26rvvev7fq2yyxleet"
  "mtst1az26c3hnh0aw5yg5cs67v7k9sg3rlmul"
)

TOTAL=${#TARGETS[@]}
COUNT=0
SUCCESS=0
FAILED=0

echo "==============================================================="
echo "Toplam $TOTAL real testnet cüzdanına 1'er SKS göndərilir..."
echo "==============================================================="

for TARGET in "${TARGETS[@]}"; do
  COUNT=$((COUNT + 1))
  echo ""
  echo "[$COUNT/$TOTAL] ==============================================="
  echo ">> Sync olunur və 1 SKS göndərilir -> $TARGET"

  # Blok budanması xətasını önləmək üçün sync
  miden-client sync >/dev/null 2>&1

  # Avtomatik 'y' təsdiqi ilə göndərmə
  if echo "y" | miden-client transfer --target "$TARGET" --asset 1::SKS --note-type public; then
    echo ">> [UĞURLU] 1 SKS -> $TARGET"
    SUCCESS=$((SUCCESS + 1))
  else
    echo ">> [XƏTA BAŞ VERDİ] $TARGET"
    FAILED=$((FAILED + 1))
  fi

  sleep 1
done

echo ""
echo "==============================================================="
echo "Bütün transferlər tamamlandı!"
echo "Uğurlu: $SUCCESS | Uğursuz: $FAILED"
echo "Son sinxronizasiya edilir..."
miden-client sync

echo ""
echo "Yenilənmiş Cüzdan Vəziyyətin:"
miden-client account -s 0x86341565987080016e478676445d61
