use std::ffi::c_int;

// SAFETY: The function declarations given below are in line with the header files of `libc`.
#[link(name = "c")]
unsafe extern "C" {

	/// `getchar()` - input of characters and strings.
	///
	/// Reads the next character from `stdin` and returns it as a `char` cast to an `i32`, or `EOF` on end of file or error.
	///
	/// # Library
	///
	/// Source(s):
	///
	/// - C Standard Library (`libc`).
	///
	/// Standard(s):
	///
	/// - [POSIX.1-2008].
	/// - [C11].
	///
	/// Declaration:
	///
	/// ```
	/// #include <stdio.h>
	///
	/// int getchar(void);
	/// ```
	///
	/// # Safety
	///
	/// C foreign function calls and I/O operations are unsafe.
	///
	/// This function is unsafe, and can potentially cause undefined behaviour if misused.
	///
	/// # Errors
	///
	/// The `getchar()` function shall fail if data needs to be read and:
	///
	/// - `EAGAIN`: The `O_NONBLOCK` flag is set for the file descriptor underlying `stdin` and the thread would be delayed in the `getchar()` operation.
	/// - `EBADF`: The file descriptor underlying `stdin` is not a valid file descriptor open for reading.
	/// - `EINTR`: The read operation was terminated due to the receipt of a signal, and no data was transferred.
	/// - `EIO`: A physical I/O error has occurred, or the process is in a background process group attempting to read from its controlling terminal, and either the process is ignoring or blocking the `SIGTTIN` signal or the process group is orphaned.  This error may also be generated for implementation-defined reasons.
	/// - `EOVERFLOW`: The file is a regular file and an attempt was made to read at or beyond the offset maximum associated with the corresponding stream.
	///
	/// The `getchar()` function may fail if:
	///
	/// - `ENOMEM`: Insufficient storage space is available.
	/// - `ENXIO`: A request was made of a nonexistent device, or the request was outside the capabilities of the device.
	///
	/// # Returns
	///
	/// Upon successful completion, `getchar()` shall return the next byte from the input stream pointed to by `stdin`. If the end-of-file indicator for `stdin` is set, or if `stdin` is at end-of-file, the end-of-file indicator for the `stdin` shall be set and `getchar()` shall return `EOF`. If a read error occurs, the error indicator for `stdin` shall be set, `getchar()` shall return `EOF`, and shall set <u>`errno`</u> to indicate the error.
	///
	/// # See Also
	///
	/// [read(2)], [write(2)], [ferror(3)], [fgetwc(3)], [fgetws(3)], [fopen(3)], [fread(3)], [fseek(3)], [getline(3)], [gets(3)], [getwchar(3)], [puts(3)], [scanf(3)], [ungetwc(3)], [unlocked_stdio(3)], [feature_test_macros(7)], [POSIX.1]
	///
	/// [POSIX.1-2008]: https://pubs.opengroup.org/onlinepubs/9699919799.2008edition/functions/getchar.html
	/// [POSIX.1]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/getchar.html
	/// [C11]: https://www.open-std.org/jtc1/sc22/wg14/www/docs/n1570.pdf
	/// [read(2)]: https://man7.org/linux/man-pages/man2/read.2.html
	/// [write(2)]: https://man7.org/linux/man-pages/man2/write.2.html
	/// [ferror(3)]: https://man7.org/linux/man-pages/man3/ferror.3.html
	/// [fgetwc(3)]: https://man7.org/linux/man-pages/man3/fgetwc.3.html
	/// [fgetws(3)]: https://man7.org/linux/man-pages/man3/fgetws.3.html
	/// [fopen(3)]: https://man7.org/linux/man-pages/man3/fopen.3.html
	/// [fread(3)]: https://man7.org/linux/man-pages/man3/fread.3.html
	/// [fseek(3)]: https://man7.org/linux/man-pages/man3/fseek.3.html
	/// [getline(3)]: https://man7.org/linux/man-pages/man3/getline.3.html
	/// [gets(3)]: https://man7.org/linux/man-pages/man3/gets.3.html
	/// [getwchar(3)]: https://man7.org/linux/man-pages/man3/getwchar.3.html
	/// [puts(3)]: https://man7.org/linux/man-pages/man3/puts.3.html
	/// [scanf(3)]: https://man7.org/linux/man-pages/man3/scanf.3.html
	/// [ungetwc(3)]: https://man7.org/linux/man-pages/man3/ungetwc.3.html
	/// [unlocked_stdio(3)]: https://man7.org/linux/man-pages/man3/unlocked_stdio.3.html
	/// [feature_test_macros(7)]: https://man7.org/linux/man-pages/man7/feature_test_macros.7.html
	#[must_use]
	pub(crate) fn getchar() -> c_int;
}
