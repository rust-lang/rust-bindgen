// bindgen-flags: --rust-target 1.0 --with-derive-hash --with-derive-partialeq --with-derive-eq --rustified-enum .*

template<typename T>
struct TErrorResult {
  enum UnionState {
    HasMessage,
    HasException,
  };
  int mResult;
  struct Message;
  struct DOMExceptionInfo;
  union {
    Message* mMessage;
    DOMExceptionInfo* mDOMExceptionInfo;
  };

  bool mMightHaveUnreported;
  UnionState mUnionState;
};

struct ErrorResult : public TErrorResult<int> {
};
