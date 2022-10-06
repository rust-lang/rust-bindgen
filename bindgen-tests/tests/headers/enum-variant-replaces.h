
/** Type for a OGR error */
typedef enum
{
    OGRERR_NONE,                       /**< Success */
    OGRERR_NOT_ENOUGH_DATA,            /**< Not enough data to deserialize */
    OGRERR_NOT_ENOUGH_MEMORY,          /**< Not enough memory */
    OGRERR_UNSUPPORTED_GEOMETRY_TYPE,  /**< Unsupported geometry type */
    OGRERR_UNSUPPORTED_OPERATION,      /**< Unsupported operation */
    OGRERR_CORRUPT_DATA,               /**< Corrupt data */
    OGRERR_FAILURE,                    /**< Failure */
    OGRERR_UNSUPPORTED_SRS,            /**< Unsupported SRS */
    OGRERR_INVALID_HANDLE,             /**< Invalid handle */
    OGRERR_NON_EXISTING_FEATURE        /**< Non existing feature. Added in GDAL 2.0 */
} OGRErr;

/**
 * <div rustbindgen replaces="OGRErr"></div>
 */
typedef enum
{
    /**
     * <div rustbindgen replaces="PASS"></div>
     *
     * Should see PASS below.
     */
    FAIL,
    /**
     * <div rustbindgen replaces="OGRERR_NONE"></div>
     *
     * Should see OGRERR_NONE instead of CUSTOM_OGRERR_NONE below.
     */
    CUSTOM_OGRERR_NONE
} StrictOGRErr;
