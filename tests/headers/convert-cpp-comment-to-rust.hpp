
typedef unsigned mbedtls_mpi_uint;

/**
 * \brief          MPI structure
 */
typedef struct
{
    int s;                /*!<  integer sign      */
    unsigned long n;      /*!<  total # of limbs  */
    mbedtls_mpi_uint *p;  /*!<  pointer to limbs  */
}
mbedtls_mpi;

