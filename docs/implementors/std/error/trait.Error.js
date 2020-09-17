(function() {var implementors = {};
implementors["adblock"] = [{"text":"impl Error for BlockerError","synthetic":false,"types":[]}];
implementors["addr"] = [{"text":"impl Error for Error","synthetic":false,"types":[]}];
implementors["aho_corasick"] = [{"text":"impl Error for Error","synthetic":false,"types":[]}];
implementors["base64"] = [{"text":"impl Error for DecodeError","synthetic":false,"types":[]}];
implementors["error_chain"] = [{"text":"impl Error for Error","synthetic":false,"types":[]},{"text":"impl Error for Error","synthetic":false,"types":[]}];
implementors["flate2"] = [{"text":"impl Error for DecompressError","synthetic":false,"types":[]},{"text":"impl Error for CompressError","synthetic":false,"types":[]}];
implementors["native_tls"] = [{"text":"impl Error for Error","synthetic":false,"types":[]},{"text":"impl&lt;S&gt; Error for HandshakeError&lt;S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: Any + Debug,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["openssl"] = [{"text":"impl Error for ErrorStack","synthetic":false,"types":[]},{"text":"impl Error for Error","synthetic":false,"types":[]},{"text":"impl Error for Error","synthetic":false,"types":[]},{"text":"impl&lt;S:&nbsp;Debug&gt; Error for HandshakeError&lt;S&gt;","synthetic":false,"types":[]},{"text":"impl Error for X509VerifyResult","synthetic":false,"types":[]}];
implementors["psl_lexer"] = [{"text":"impl Error for Error","synthetic":false,"types":[]}];
implementors["regex"] = [{"text":"impl Error for Error","synthetic":false,"types":[]}];
implementors["regex_syntax"] = [{"text":"impl Error for Error","synthetic":false,"types":[]},{"text":"impl Error for Error","synthetic":false,"types":[]},{"text":"impl Error for Error","synthetic":false,"types":[]},{"text":"impl Error for CaseFoldError","synthetic":false,"types":[]},{"text":"impl Error for UnicodeWordError","synthetic":false,"types":[]}];
implementors["rmp"] = [{"text":"impl&lt;'a&gt; Error for DecodeStringError&lt;'a&gt;","synthetic":false,"types":[]},{"text":"impl Error for ValueReadError","synthetic":false,"types":[]},{"text":"impl Error for NumValueReadError","synthetic":false,"types":[]},{"text":"impl Error for ValueWriteError","synthetic":false,"types":[]}];
implementors["rmp_serde"] = [{"text":"impl Error for Error","synthetic":false,"types":[]},{"text":"impl Error for Error","synthetic":false,"types":[]}];
implementors["serde"] = [{"text":"impl Error for Error","synthetic":false,"types":[]}];
implementors["syn"] = [{"text":"impl Error for ParseError","synthetic":false,"types":[]}];
implementors["url"] = [{"text":"impl Error for ParseError","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()