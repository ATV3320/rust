(function() {var implementors = {
"either":[["impl&lt;L, R&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.66.0/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a> for <a class=\"enum\" href=\"either/enum.Either.html\" title=\"enum either::Either\">Either</a>&lt;L, R&gt;<span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;L: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.66.0/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;R: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.66.0/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a>&lt;Output = L::<a class=\"associatedtype\" href=\"https://doc.rust-lang.org/1.66.0/core/future/future/trait.Future.html#associatedtype.Output\" title=\"type core::future::future::Future::Output\">Output</a>&gt;,</span>"]],
"flume":[["impl&lt;'a, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.66.0/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a> for <a class=\"struct\" href=\"flume/async/struct.SendFut.html\" title=\"struct flume::async::SendFut\">SendFut</a>&lt;'a, T&gt;"],["impl&lt;'a, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.66.0/core/future/future/trait.Future.html\" title=\"trait core::future::future::Future\">Future</a> for <a class=\"struct\" href=\"flume/async/struct.RecvFut.html\" title=\"struct flume::async::RecvFut\">RecvFut</a>&lt;'a, T&gt;"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()