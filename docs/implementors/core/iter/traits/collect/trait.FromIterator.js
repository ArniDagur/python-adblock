(function() {var implementors = {};
implementors["adblock"] = [{"text":"impl FromIterator&lt;NetworkFilterMask&gt; for NetworkFilterMask","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;CosmeticFilterMask&gt; for CosmeticFilterMask","synthetic":false,"types":[]}];
implementors["hashbrown"] = [{"text":"impl&lt;K, V, S&gt; FromIterator&lt;(K, V)&gt; for HashMap&lt;K, V, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;K: Eq + Hash,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: BuildHasher + Default,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T, S&gt; FromIterator&lt;T&gt; for HashSet&lt;T, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Eq + Hash,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: BuildHasher + Default,&nbsp;</span>","synthetic":false,"types":[]}];
implementors["openssl"] = [{"text":"impl FromIterator&lt;CMSOptions&gt; for CMSOptions","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;OcspFlag&gt; for OcspFlag","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;Pkcs7Flags&gt; for Pkcs7Flags","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;SslOptions&gt; for SslOptions","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;SslMode&gt; for SslMode","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;SslVerifyMode&gt; for SslVerifyMode","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;SslSessionCacheMode&gt; for SslSessionCacheMode","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;ExtensionContext&gt; for ExtensionContext","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;ShutdownState&gt; for ShutdownState","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;X509CheckFlags&gt; for X509CheckFlags","synthetic":false,"types":[]}];
implementors["proc_macro2"] = [{"text":"impl FromIterator&lt;TokenTree&gt; for TokenStream","synthetic":false,"types":[]},{"text":"impl FromIterator&lt;TokenStream&gt; for TokenStream","synthetic":false,"types":[]}];
implementors["smallvec"] = [{"text":"impl&lt;A:&nbsp;Array&gt; FromIterator&lt;&lt;A as Array&gt;::Item&gt; for SmallVec&lt;A&gt;","synthetic":false,"types":[]}];
implementors["syn"] = [{"text":"impl&lt;T, P&gt; FromIterator&lt;T&gt; for Punctuated&lt;T, P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;P: Default,&nbsp;</span>","synthetic":false,"types":[]},{"text":"impl&lt;T, P&gt; FromIterator&lt;Pair&lt;T, P&gt;&gt; for Punctuated&lt;T, P&gt;","synthetic":false,"types":[]}];
implementors["tinyvec"] = [{"text":"impl&lt;A:&nbsp;Array + Default&gt; FromIterator&lt;&lt;A as Array&gt;::Item&gt; for ArrayVec&lt;A&gt;","synthetic":false,"types":[]},{"text":"impl&lt;A:&nbsp;Array + Default&gt; FromIterator&lt;&lt;A as Array&gt;::Item&gt; for TinyVec&lt;A&gt;","synthetic":false,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()