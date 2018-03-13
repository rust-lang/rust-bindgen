// bindgen-flags: --rustified-enum '.*'


typedef enum MyDupeEnum {
	A = 0,
	A_alias = 0,
	B,
} MyDupeEnum;

enum MyOtherDupeEnum {
	C = 0,
	C_alias = 0,
	D,
};