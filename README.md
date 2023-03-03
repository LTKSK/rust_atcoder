atcoderで競プロを勉強するためのレポジトリ

# keywards

## マンハッタン距離

ある座標2点がある時、|x1 - x2| + |y1 - y2| で計算される距離

特定の座標からの距離をプロットするとピラミッドのような形になる

## 部分和問題と考え方

N個の整数a0..aNがある時、これらの任意の組みわせでWを作れればYesといった問題

この場合の組み合わせは2^N個あって、例えばN=3の時は8個存在する (空集合も含む)

この時整数の2進数表現を使う。N桁の2進数で、すべての組み合わせを列挙出来る

例えばN=3でa1とa3ならば101,といった感じ  
この2進数は整数として表現可能でかつ、その値は2^N未満である(N++毎に桁が増えるからね)

n番目のbitが立っているかは、bit & (1 << n)で確認が出来る

これで何が便利かと言うと、組み合わせを羅列する時に便利  
bitは(1をNで左シフトした値)まで回すと、2^Nの範囲全てを舐めるみたい

## ユークリッド互除法

GCD(m,n)はmとnの最大公約数を求める関数。これをユークリッド互除法を使って解く  
最大公約数の性質としてm%nをrとすると、  
GCD(m,n) == GCD(n, r)らしい

m,nからn,rが求められるので。n,rを次のm,nとしてあまりが0になるまで再帰する、  
というわけ

## 整数計算出来ないか調べる(floatに注意！)

例えばxミリリットルのy%のアルコールとかになった時に、
いつもだとx*y/100.0とかしたくなる
けどこの時上の計算の比較対象を100倍すれば整数だけで収まる
小数は可能なら使わない方が良さそうだ
なぜかと言うと、浮動小数点数計算は誤差を含むから

## indexに注意

文中のi番目は1originだけど、プログラムで配列アクセスする時は、  
0originなので注意が要る

## intの範囲に注意

結構足りなくなるぞ！

## loopの順番に注意

2つの要素がx<=yのような関係の時、
最初に外側のloopに来るのはxの方

## 問題文に注意

ほんとにnか確認すること。2^N人だったりするぞ
2^nは1<<Nとすると楽

## 座標圧縮
0~10^9の数直線ってメモリに乗らないよね～
というわけで圧縮しねぇと計算どころじゃねぇ！
ということで、要素の大きさでsortして前からn番みたいな、
nを割り振っていく(左からn番目と考えてもよい?)


## いもす法
あるイベントの変化のタイミング
（例えば料金が上がる/下がる）
のみをカウントして後で全体を計算する手法、のようだ?

## String > num
string.parse::<T>().unwrap()
charの場合はto_stringをかませる

## 重複を排除する時はHashSetかHashMap

## 各桁の情報を知る

10進数だったらば、n%10を求めたら良い
求めたあとに n/= 10とすれば次の桁に進む
8進数でもやることは同じ
この時扱う数値に注意

## 累積和
s0 = 0
s1 = a0
s2 = a0 + a1...
sN = a0...+aN-1
s0は0である。これはsNのNが各数値の隙間を表すイメージなため

## rustのwindows

forなどで、現在と一つ後ろの情報に触りたいときとかに使える。
for (p, n) in vec.windows(2){}

## 一次合同方程式、合同方程式

aとbはpを法として合同、とはa,bそれぞれをpで割ったあまりが等しいの意
定義
a,bは整数、pは正の整数とした時、a-bがpで割り切れるならばaとbはpをmodulus（法）
として合同であると呼び、a≡b(mod p)と書く
modはmodulo（測定、尺度）といった意味

modによって、無限個ある整数が有限個（p-1）の範囲になる
同じ法で合同なものは、和差積それぞれとっても同じ操作

a≡b(mod p),c≡d(mod p)の時、
a+c ≡ b+d,
a-c ≡ b-d,
ac ≡ bd,
除法だけは条件が必要
ma ≡ mb(mod p)の時、mとpがお互いに素であるときだけ成立する

例題). 3^26/8のあまりを求めよ
3^26 ≡ (mod 8)
(3^2)^13と考える
すると≡9^13であり、≡1^13である
これはa^n≡b^n (mod p)であることにより,9,1それぞれがmod 8の元に合同だからである
1^13=1なので、求める余りは1
余りが1か-1になる数を探すのが肝

## 1次合同方程式
ax≡b(mod m)のxを求める方程式
(a,m) = 1の時はただ一つ解がある。(a,m) = d > 1の時はbがdで割り切れる時に限って解がある
その数はd個
ナイーブにやると、0..m-1を順にxとして計算すれば良いそうな
(ただしもちろん数値がでかければ現実的な時間では終わらない)

## 互いに素であるとは
a,bをともに割り切る正の整数が1のみであることを言う
すなわちa,bの最大公約数gcd(a,b)==1ということ

## gcd最大公約数
m,nがある時、求め方はGCD(m,n) => GCD(n, m mod n)...

## 複数要素がある時は、片方を固定して考える

例えば3つの配列が出てきたときとかは、一つを固定して考えてみると吉

## superslice

upper_boundとlower_boundが使える
upper_bound => key以上の値を持つ最小のindex
lower_bound => keyより大きい値を持つ最小のindex

## lower_boundの二分探索をやる時は、left=-1から

## 最小公倍数Lの求め方
A,Bの最大公約数をGとし、最小公倍数をLとすると、
AB=GL
よってL=AB/G
一般的には最小公倍数はlcmと書くのだそうな

## NからAでもBでもない数
みたいな表現をされる時は、集合っぽく考えると良さそう

## 値が変動する場合、上限下限を意識
maxやminを取る
また、aからbまでみたいな区間が複数登場する場合、
時系列であればbを記録して次のaと比較などとするとシンプル

## 順列と組み合わせ
並べ方を区別するのが順列
区別しないのが組み合わせ
nPr = n! / (n-r)!
5P3なら5*4*3 = n(n-1)...(n - r + 1)
PはpermutationのP

nCr = nPr / r!

## numクレート
div_floor>切り捨て
div_ceil>切り上げなどがある

## UnionFind

グループ分けを効率的に管理するデータ構造。根付き木構造
・以下のqueryに対して高速
- 要素x,yが同じグループに属するか調べる
- 要素xを含むグループと要素yを含むグループを併合する

二分木ではない。グループ毎にrootを持つ
root関数は、要素xを含むグループのrootを返す関数で。頂点xからparentを辿ってみたいな処理らしい。
処理時間はO(h)。hは高さ
insame（グループ一致判定）は、root(x)==root(y)かどうかで判定している

union処理時に際して、小さい方のrootを新しいrootとすることで、各木の高さをlogNで抑えられる

## LSB
最下位bitのこと

## XORの性質について

前後を入れ替えても変わらない
n XOR nは0

☆奇数の時、x+1/2が偶数なら0、奇数なら1になる

## 動的計画法

全体の計算を小さな計算に分けて最終的な結果を求める
- 1回に移動距離の違う2つの部屋いずれかを選んで移動できる時の最小移動コストを求めるとか

最後の行動で場合分けするのを`貰う遷移形式`と呼ぶ  
逆に一手先の行動を考えて配列を更新するのを`配る遷移形式`と呼ぶ

dpの初期値はmaxを取る場合は小さな値にしておくと良い。0で初期化するとうまくいかないときがある

## 累積和

TODO

## 深さ優先探索(DFS)

## 幅優先探索(BFS)

- 探索先からつながるnodeをqueueに詰めていくことで探索を行う
- 二次元の移動は、コスト1の移動として考えられる 
- 探索済みはflagを立てるか、初期値-1などにして区別できるようにすると良い

## 1000000007

でっかい素数。10000000007以下同士で足しても32bit超えない。掛けても64bit超えない

a*b*c mod nは、(a*b mod n) * c mod nでも同じ

☆ pを素数とし、bをpで割り切れない整数とする。この時aを整数として、
bx≡a(mod p)を満たすようなxはmod p において一意に存在する

## mod pにおける逆元

a/b (mod p)を計算する方法
1 / b (mod p)が計算できれば良い。なぜなら、
a / b ≡ a * (1/b) (mod p) が成り立つため。ここで1/bをmod pにおけるbの逆元と呼ぶ。
pを法とする世界で、b倍すると1になる数
例えばp=13の世界では2の逆元は7(2*14%13 == 1)

## graphは頂点にアクセスするすべての情報に注意
上記のUsize1で置き換えるのが良い。各値が何を指す値なのか注意すること

## 隣接行列
頂点数*頂点数の要素を持つ行列。booleanを入れて、Trueである行列のindex、m[x][y]があった時、
xとyにはedgeがあるとみなす

# Rustでやるときの知識

## proconioのfastoutとUsize1
fastoutはmainにつけるとprintlnが早くなるらしい
Usize1は、edgeとかを受ける時に使うと、1basedな値を0basedに置き換えてくれる

