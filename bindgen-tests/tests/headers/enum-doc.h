/** Document enum */
enum B {
    /// Document field with three slashes
    VAR_A = 0,
    /** Document field with preceding star */
    VAR_B = 1,
    /*! Document field with preceding exclamation */
    VAR_C = 2,
    VAR_D = 3, /**< Document field with following star */
    VAR_E = 4, /*!< Document field with following exclamation */
    /**
     * Document field with preceding star, with a loong long multiline
     * comment.
     *
     * Very interesting documentation, definitely.
     */
    VAR_F,
};
