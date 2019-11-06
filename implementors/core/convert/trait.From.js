(function() {var implementors = {};
implementors["quaint"] = [{text:"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"quaint/ast/struct.Column.html\" title=\"struct quaint::ast::Column\">Column</a>&lt;'a&gt;&gt; for <a class=\"enum\" href=\"quaint/ast/enum.DatabaseValue.html\" title=\"enum quaint::ast::DatabaseValue\">DatabaseValue</a>&lt;'a&gt;",synthetic:false,types:["quaint::ast::values::DatabaseValue"]},{text:"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;&amp;'a <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.str.html\">str</a>&gt; for <a class=\"struct\" href=\"quaint/ast/struct.Column.html\" title=\"struct quaint::ast::Column\">Column</a>&lt;'a&gt;",synthetic:false,types:["quaint::ast::column::Column"]},{text:"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>&gt; for <a class=\"struct\" href=\"quaint/ast/struct.Column.html\" title=\"struct quaint::ast::Column\">Column</a>&lt;'a&gt;",synthetic:false,types:["quaint::ast::column::Column"]},{text:"impl&lt;'a, T, C&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\">(</a>T, C<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\">)</a>&gt; for <a class=\"struct\" href=\"quaint/ast/struct.Column.html\" title=\"struct quaint::ast::Column\">Column</a>&lt;'a&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"struct\" href=\"quaint/ast/struct.Table.html\" title=\"struct quaint::ast::Table\">Table</a>&lt;'a&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;C: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"struct\" href=\"quaint/ast/struct.Column.html\" title=\"struct quaint::ast::Column\">Column</a>&lt;'a&gt;&gt;,&nbsp;</span>",synthetic:false,types:["quaint::ast::column::Column"]},{text:"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"quaint/ast/enum.Compare.html\" title=\"enum quaint::ast::Compare\">Compare</a>&lt;'a&gt;&gt; for <a class=\"enum\" href=\"quaint/ast/enum.ConditionTree.html\" title=\"enum quaint::ast::ConditionTree\">ConditionTree</a>&lt;'a&gt;",synthetic:false,types:["quaint::ast::conditions::ConditionTree"]},{text:"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"quaint/ast/enum.Compare.html\" title=\"enum quaint::ast::Compare\">Compare</a>&lt;'a&gt;&gt; for <a class=\"enum\" href=\"quaint/ast/enum.Expression.html\" title=\"enum quaint::ast::Expression\">Expression</a>&lt;'a&gt;",synthetic:false,types:["quaint::ast::expression::Expression"]},{text:"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"quaint/ast/enum.ConditionTree.html\" title=\"enum quaint::ast::ConditionTree\">ConditionTree</a>&lt;'a&gt;&gt; for <a class=\"enum\" href=\"quaint/ast/enum.Expression.html\" title=\"enum quaint::ast::Expression\">Expression</a>&lt;'a&gt;",synthetic:false,types:["quaint::ast::expression::Expression"]},{text:"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"quaint/ast/struct.Select.html\" title=\"struct quaint::ast::Select\">Select</a>&lt;'a&gt;&gt; for <a class=\"enum\" href=\"quaint/ast/enum.ConditionTree.html\" title=\"enum quaint::ast::ConditionTree\">ConditionTree</a>&lt;'a&gt;",synthetic:false,types:["quaint::ast::conditions::ConditionTree"]},{text:"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"quaint/ast/struct.Delete.html\" title=\"struct quaint::ast::Delete\">Delete</a>&lt;'a&gt;&gt; for <a class=\"enum\" href=\"quaint/ast/enum.Query.html\" title=\"enum quaint::ast::Query\">Query</a>&lt;'a&gt;",synthetic:false,types:["quaint::ast::query::Query"]},{text:"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"quaint/ast/struct.Select.html\" title=\"struct quaint::ast::Select\">Select</a>&lt;'a&gt;&gt; for <a class=\"enum\" href=\"quaint/ast/enum.Expression.html\" title=\"enum quaint::ast::Expression\">Expression</a>&lt;'a&gt;",synthetic:false,types:["quaint::ast::expression::Expression"]},{text:"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"quaint/ast/struct.RowNumber.html\" title=\"struct quaint::ast::RowNumber\">RowNumber</a>&lt;'a&gt;&gt; for <a class=\"struct\" href=\"quaint/ast/struct.Function.html\" title=\"struct quaint::ast::Function\">Function</a>&lt;'a&gt;",synthetic:false,types:["quaint::ast::function::Function"]},{text:"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"quaint/ast/struct.RowNumber.html\" title=\"struct quaint::ast::RowNumber\">RowNumber</a>&lt;'a&gt;&gt; for <a class=\"enum\" href=\"quaint/ast/enum.DatabaseValue.html\" title=\"enum quaint::ast::DatabaseValue\">DatabaseValue</a>&lt;'a&gt;",synthetic:false,types:["quaint::ast::values::DatabaseValue"]},{text:"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"quaint/ast/struct.Count.html\" title=\"struct quaint::ast::Count\">Count</a>&lt;'a&gt;&gt; for <a class=\"struct\" href=\"quaint/ast/struct.Function.html\" title=\"struct quaint::ast::Function\">Function</a>&lt;'a&gt;",synthetic:false,types:["quaint::ast::function::Function"]},{text:"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"quaint/ast/struct.Count.html\" title=\"struct quaint::ast::Count\">Count</a>&lt;'a&gt;&gt; for <a class=\"enum\" href=\"quaint/ast/enum.DatabaseValue.html\" title=\"enum quaint::ast::DatabaseValue\">DatabaseValue</a>&lt;'a&gt;",synthetic:false,types:["quaint::ast::values::DatabaseValue"]},{text:"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"quaint/ast/struct.AggregateToString.html\" title=\"struct quaint::ast::AggregateToString\">AggregateToString</a>&lt;'a&gt;&gt; for <a class=\"struct\" href=\"quaint/ast/struct.Function.html\" title=\"struct quaint::ast::Function\">Function</a>&lt;'a&gt;",synthetic:false,types:["quaint::ast::function::Function"]},{text:"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"quaint/ast/struct.AggregateToString.html\" title=\"struct quaint::ast::AggregateToString\">AggregateToString</a>&lt;'a&gt;&gt; for <a class=\"enum\" href=\"quaint/ast/enum.DatabaseValue.html\" title=\"enum quaint::ast::DatabaseValue\">DatabaseValue</a>&lt;'a&gt;",synthetic:false,types:["quaint::ast::values::DatabaseValue"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; for <a class=\"enum\" href=\"quaint/ast/enum.Id.html\" title=\"enum quaint::ast::Id\">Id</a>",synthetic:false,types:["quaint::ast::id::Id"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u64.html\">u64</a>&gt; for <a class=\"enum\" href=\"quaint/ast/enum.Id.html\" title=\"enum quaint::ast::Id\">Id</a>",synthetic:false,types:["quaint::ast::id::Id"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>&gt; for <a class=\"enum\" href=\"quaint/ast/enum.Id.html\" title=\"enum quaint::ast::Id\">Id</a>",synthetic:false,types:["quaint::ast::id::Id"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://docs.rs/uuid/0.7.4/uuid/struct.Uuid.html\" title=\"struct uuid::Uuid\">Uuid</a>&gt; for <a class=\"enum\" href=\"quaint/ast/enum.Id.html\" title=\"enum quaint::ast::Id\">Id</a>",synthetic:false,types:["quaint::ast::id::Id"]},{text:"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"quaint/ast/struct.Insert.html\" title=\"struct quaint::ast::Insert\">Insert</a>&lt;'a&gt;&gt; for <a class=\"enum\" href=\"quaint/ast/enum.Query.html\" title=\"enum quaint::ast::Query\">Query</a>&lt;'a&gt;",synthetic:false,types:["quaint::ast::query::Query"]},{text:"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"quaint/ast/struct.SingleRowInsert.html\" title=\"struct quaint::ast::SingleRowInsert\">SingleRowInsert</a>&lt;'a&gt;&gt; for <a class=\"struct\" href=\"quaint/ast/struct.Insert.html\" title=\"struct quaint::ast::Insert\">Insert</a>&lt;'a&gt;",synthetic:false,types:["quaint::ast::insert::Insert"]},{text:"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"quaint/ast/struct.MultiRowInsert.html\" title=\"struct quaint::ast::MultiRowInsert\">MultiRowInsert</a>&lt;'a&gt;&gt; for <a class=\"struct\" href=\"quaint/ast/struct.Insert.html\" title=\"struct quaint::ast::Insert\">Insert</a>&lt;'a&gt;",synthetic:false,types:["quaint::ast::insert::Insert"]},{text:"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"quaint/ast/struct.SingleRowInsert.html\" title=\"struct quaint::ast::SingleRowInsert\">SingleRowInsert</a>&lt;'a&gt;&gt; for <a class=\"enum\" href=\"quaint/ast/enum.Query.html\" title=\"enum quaint::ast::Query\">Query</a>&lt;'a&gt;",synthetic:false,types:["quaint::ast::query::Query"]},{text:"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"quaint/ast/struct.MultiRowInsert.html\" title=\"struct quaint::ast::MultiRowInsert\">MultiRowInsert</a>&lt;'a&gt;&gt; for <a class=\"enum\" href=\"quaint/ast/enum.Query.html\" title=\"enum quaint::ast::Query\">Query</a>&lt;'a&gt;",synthetic:false,types:["quaint::ast::query::Query"]},{text:"impl&lt;'a, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;T&gt; for <a class=\"enum\" href=\"quaint/ast/enum.Query.html\" title=\"enum quaint::ast::Query\">Query</a>&lt;'a&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/alloc/borrow/enum.Cow.html\" title=\"enum alloc::borrow::Cow\">Cow</a>&lt;'a, <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.str.html\">str</a>&gt;&gt;,&nbsp;</span>",synthetic:false,types:["quaint::ast::query::Query"]},{text:"impl&lt;'a, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;T&gt;&gt; for <a class=\"struct\" href=\"quaint/ast/struct.Row.html\" title=\"struct quaint::ast::Row\">Row</a>&lt;'a&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"enum\" href=\"quaint/ast/enum.DatabaseValue.html\" title=\"enum quaint::ast::DatabaseValue\">DatabaseValue</a>&lt;'a&gt;&gt;,&nbsp;</span>",synthetic:false,types:["quaint::ast::row::Row"]},{text:"impl&lt;'a, A&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\">(</a>A<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\">,)</a>&gt; for <a class=\"struct\" href=\"quaint/ast/struct.Row.html\" title=\"struct quaint::ast::Row\">Row</a>&lt;'a&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"enum\" href=\"quaint/ast/enum.DatabaseValue.html\" title=\"enum quaint::ast::DatabaseValue\">DatabaseValue</a>&lt;'a&gt;&gt;,&nbsp;</span>",synthetic:false,types:["quaint::ast::row::Row"]},{text:"impl&lt;'a, A, B&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\">(</a>A, B<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\">)</a>&gt; for <a class=\"struct\" href=\"quaint/ast/struct.Row.html\" title=\"struct quaint::ast::Row\">Row</a>&lt;'a&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"enum\" href=\"quaint/ast/enum.DatabaseValue.html\" title=\"enum quaint::ast::DatabaseValue\">DatabaseValue</a>&lt;'a&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;B: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"enum\" href=\"quaint/ast/enum.DatabaseValue.html\" title=\"enum quaint::ast::DatabaseValue\">DatabaseValue</a>&lt;'a&gt;&gt;,&nbsp;</span>",synthetic:false,types:["quaint::ast::row::Row"]},{text:"impl&lt;'a, A, B, C&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\">(</a>A, B, C<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\">)</a>&gt; for <a class=\"struct\" href=\"quaint/ast/struct.Row.html\" title=\"struct quaint::ast::Row\">Row</a>&lt;'a&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"enum\" href=\"quaint/ast/enum.DatabaseValue.html\" title=\"enum quaint::ast::DatabaseValue\">DatabaseValue</a>&lt;'a&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;B: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"enum\" href=\"quaint/ast/enum.DatabaseValue.html\" title=\"enum quaint::ast::DatabaseValue\">DatabaseValue</a>&lt;'a&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;C: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"enum\" href=\"quaint/ast/enum.DatabaseValue.html\" title=\"enum quaint::ast::DatabaseValue\">DatabaseValue</a>&lt;'a&gt;&gt;,&nbsp;</span>",synthetic:false,types:["quaint::ast::row::Row"]},{text:"impl&lt;'a, A, B, C, D&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\">(</a>A, B, C, D<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\">)</a>&gt; for <a class=\"struct\" href=\"quaint/ast/struct.Row.html\" title=\"struct quaint::ast::Row\">Row</a>&lt;'a&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"enum\" href=\"quaint/ast/enum.DatabaseValue.html\" title=\"enum quaint::ast::DatabaseValue\">DatabaseValue</a>&lt;'a&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;B: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"enum\" href=\"quaint/ast/enum.DatabaseValue.html\" title=\"enum quaint::ast::DatabaseValue\">DatabaseValue</a>&lt;'a&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;C: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"enum\" href=\"quaint/ast/enum.DatabaseValue.html\" title=\"enum quaint::ast::DatabaseValue\">DatabaseValue</a>&lt;'a&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;D: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"enum\" href=\"quaint/ast/enum.DatabaseValue.html\" title=\"enum quaint::ast::DatabaseValue\">DatabaseValue</a>&lt;'a&gt;&gt;,&nbsp;</span>",synthetic:false,types:["quaint::ast::row::Row"]},{text:"impl&lt;'a, A, B, C, D, E&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\">(</a>A, B, C, D, E<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\">)</a>&gt; for <a class=\"struct\" href=\"quaint/ast/struct.Row.html\" title=\"struct quaint::ast::Row\">Row</a>&lt;'a&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"enum\" href=\"quaint/ast/enum.DatabaseValue.html\" title=\"enum quaint::ast::DatabaseValue\">DatabaseValue</a>&lt;'a&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;B: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"enum\" href=\"quaint/ast/enum.DatabaseValue.html\" title=\"enum quaint::ast::DatabaseValue\">DatabaseValue</a>&lt;'a&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;C: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"enum\" href=\"quaint/ast/enum.DatabaseValue.html\" title=\"enum quaint::ast::DatabaseValue\">DatabaseValue</a>&lt;'a&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;D: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"enum\" href=\"quaint/ast/enum.DatabaseValue.html\" title=\"enum quaint::ast::DatabaseValue\">DatabaseValue</a>&lt;'a&gt;&gt;,<br>&nbsp;&nbsp;&nbsp;&nbsp;E: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"enum\" href=\"quaint/ast/enum.DatabaseValue.html\" title=\"enum quaint::ast::DatabaseValue\">DatabaseValue</a>&lt;'a&gt;&gt;,&nbsp;</span>",synthetic:false,types:["quaint::ast::row::Row"]},{text:"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"quaint/ast/struct.Select.html\" title=\"struct quaint::ast::Select\">Select</a>&lt;'a&gt;&gt; for <a class=\"enum\" href=\"quaint/ast/enum.DatabaseValue.html\" title=\"enum quaint::ast::DatabaseValue\">DatabaseValue</a>&lt;'a&gt;",synthetic:false,types:["quaint::ast::values::DatabaseValue"]},{text:"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"quaint/ast/struct.Select.html\" title=\"struct quaint::ast::Select\">Select</a>&lt;'a&gt;&gt; for <a class=\"enum\" href=\"quaint/ast/enum.Query.html\" title=\"enum quaint::ast::Query\">Query</a>&lt;'a&gt;",synthetic:false,types:["quaint::ast::query::Query"]},{text:"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;&amp;'a <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.str.html\">str</a>&gt; for <a class=\"struct\" href=\"quaint/ast/struct.Table.html\" title=\"struct quaint::ast::Table\">Table</a>&lt;'a&gt;",synthetic:false,types:["quaint::ast::table::Table"]},{text:"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\">(</a>&amp;'a <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.str.html\">str</a>, &amp;'a <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.str.html\">str</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\">)</a>&gt; for <a class=\"struct\" href=\"quaint/ast/struct.Table.html\" title=\"struct quaint::ast::Table\">Table</a>&lt;'a&gt;",synthetic:false,types:["quaint::ast::table::Table"]},{text:"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>&gt; for <a class=\"struct\" href=\"quaint/ast/struct.Table.html\" title=\"struct quaint::ast::Table\">Table</a>&lt;'a&gt;",synthetic:false,types:["quaint::ast::table::Table"]},{text:"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\">(</a><a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a><a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.tuple.html\">)</a>&gt; for <a class=\"struct\" href=\"quaint/ast/struct.Table.html\" title=\"struct quaint::ast::Table\">Table</a>&lt;'a&gt;",synthetic:false,types:["quaint::ast::table::Table"]},{text:"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"quaint/ast/struct.Select.html\" title=\"struct quaint::ast::Select\">Select</a>&lt;'a&gt;&gt; for <a class=\"struct\" href=\"quaint/ast/struct.Table.html\" title=\"struct quaint::ast::Table\">Table</a>&lt;'a&gt;",synthetic:false,types:["quaint::ast::table::Table"]},{text:"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"quaint/ast/struct.Select.html\" title=\"struct quaint::ast::Select\">Select</a>&lt;'a&gt;&gt; for <a class=\"struct\" href=\"quaint/ast/struct.UnionAll.html\" title=\"struct quaint::ast::UnionAll\">UnionAll</a>&lt;'a&gt;",synthetic:false,types:["quaint::ast::union_all::UnionAll"]},{text:"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"quaint/ast/struct.UnionAll.html\" title=\"struct quaint::ast::UnionAll\">UnionAll</a>&lt;'a&gt;&gt; for <a class=\"enum\" href=\"quaint/ast/enum.Query.html\" title=\"enum quaint::ast::Query\">Query</a>&lt;'a&gt;",synthetic:false,types:["quaint::ast::query::Query"]},{text:"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"quaint/ast/struct.Update.html\" title=\"struct quaint::ast::Update\">Update</a>&lt;'a&gt;&gt; for <a class=\"enum\" href=\"quaint/ast/enum.Query.html\" title=\"enum quaint::ast::Query\">Query</a>&lt;'a&gt;",synthetic:false,types:["quaint::ast::query::Query"]},{text:"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"quaint/ast/enum.ParameterizedValue.html\" title=\"enum quaint::ast::ParameterizedValue\">ParameterizedValue</a>&lt;'a&gt;&gt; for <a class=\"enum\" href=\"https://docs.rs/serde_json/1.0.41/serde_json/value/enum.Value.html\" title=\"enum serde_json::value::Value\">Value</a>",synthetic:false,types:["serde_json::value::Value"]},{text:"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;&amp;'a <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.str.html\">str</a>&gt; for <a class=\"enum\" href=\"quaint/ast/enum.ParameterizedValue.html\" title=\"enum quaint::ast::ParameterizedValue\">ParameterizedValue</a>&lt;'a&gt;",synthetic:false,types:["quaint::ast::values::ParameterizedValue"]},{text:"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.String.html\" title=\"struct alloc::string::String\">String</a>&gt; for <a class=\"enum\" href=\"quaint/ast/enum.ParameterizedValue.html\" title=\"enum quaint::ast::ParameterizedValue\">ParameterizedValue</a>&lt;'a&gt;",synthetic:false,types:["quaint::ast::values::ParameterizedValue"]},{text:"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt; for <a class=\"enum\" href=\"quaint/ast/enum.ParameterizedValue.html\" title=\"enum quaint::ast::ParameterizedValue\">ParameterizedValue</a>&lt;'a&gt;",synthetic:false,types:["quaint::ast::values::ParameterizedValue"]},{text:"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i32.html\">i32</a>&gt; for <a class=\"enum\" href=\"quaint/ast/enum.ParameterizedValue.html\" title=\"enum quaint::ast::ParameterizedValue\">ParameterizedValue</a>&lt;'a&gt;",synthetic:false,types:["quaint::ast::values::ParameterizedValue"]},{text:"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i64.html\">i64</a>&gt; for <a class=\"enum\" href=\"quaint/ast/enum.ParameterizedValue.html\" title=\"enum quaint::ast::ParameterizedValue\">ParameterizedValue</a>&lt;'a&gt;",synthetic:false,types:["quaint::ast::values::ParameterizedValue"]},{text:"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.f64.html\">f64</a>&gt; for <a class=\"enum\" href=\"quaint/ast/enum.ParameterizedValue.html\" title=\"enum quaint::ast::ParameterizedValue\">ParameterizedValue</a>&lt;'a&gt;",synthetic:false,types:["quaint::ast::values::ParameterizedValue"]},{text:"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a>&gt; for <a class=\"enum\" href=\"quaint/ast/enum.ParameterizedValue.html\" title=\"enum quaint::ast::ParameterizedValue\">ParameterizedValue</a>&lt;'a&gt;",synthetic:false,types:["quaint::ast::values::ParameterizedValue"]},{text:"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"https://docs.rs/serde_json/1.0.41/serde_json/value/enum.Value.html\" title=\"enum serde_json::value::Value\">Value</a>&gt; for <a class=\"enum\" href=\"quaint/ast/enum.ParameterizedValue.html\" title=\"enum quaint::ast::ParameterizedValue\">ParameterizedValue</a>&lt;'a&gt;",synthetic:false,types:["quaint::ast::values::ParameterizedValue"]},{text:"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://docs.rs/uuid/0.7.4/uuid/struct.Uuid.html\" title=\"struct uuid::Uuid\">Uuid</a>&gt; for <a class=\"enum\" href=\"quaint/ast/enum.ParameterizedValue.html\" title=\"enum quaint::ast::ParameterizedValue\">ParameterizedValue</a>&lt;'a&gt;",synthetic:false,types:["quaint::ast::values::ParameterizedValue"]},{text:"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://docs.rs/chrono/latest/chrono/datetime/struct.DateTime.html\" title=\"struct chrono::datetime::DateTime\">DateTime</a>&lt;<a class=\"struct\" href=\"https://docs.rs/chrono/latest/chrono/offset/utc/struct.Utc.html\" title=\"struct chrono::offset::utc::Utc\">Utc</a>&gt;&gt; for <a class=\"enum\" href=\"quaint/ast/enum.ParameterizedValue.html\" title=\"enum quaint::ast::ParameterizedValue\">ParameterizedValue</a>&lt;'a&gt;",synthetic:false,types:["quaint::ast::values::ParameterizedValue"]},{text:"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"quaint/ast/struct.Row.html\" title=\"struct quaint::ast::Row\">Row</a>&lt;'a&gt;&gt; for <a class=\"enum\" href=\"quaint/ast/enum.DatabaseValue.html\" title=\"enum quaint::ast::DatabaseValue\">DatabaseValue</a>&lt;'a&gt;",synthetic:false,types:["quaint::ast::values::DatabaseValue"]},{text:"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"quaint/ast/struct.Function.html\" title=\"struct quaint::ast::Function\">Function</a>&lt;'a&gt;&gt; for <a class=\"enum\" href=\"quaint/ast/enum.DatabaseValue.html\" title=\"enum quaint::ast::DatabaseValue\">DatabaseValue</a>&lt;'a&gt;",synthetic:false,types:["quaint::ast::values::DatabaseValue"]},{text:"impl&lt;'a, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;T&gt; for <a class=\"enum\" href=\"quaint/ast/enum.DatabaseValue.html\" title=\"enum quaint::ast::DatabaseValue\">DatabaseValue</a>&lt;'a&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"enum\" href=\"quaint/ast/enum.ParameterizedValue.html\" title=\"enum quaint::ast::ParameterizedValue\">ParameterizedValue</a>&lt;'a&gt;&gt;,&nbsp;</span>",synthetic:false,types:["quaint::ast::values::DatabaseValue"]},{text:"impl&lt;'a, T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;T&gt;&gt; for <a class=\"enum\" href=\"quaint/ast/enum.DatabaseValue.html\" title=\"enum quaint::ast::DatabaseValue\">DatabaseValue</a>&lt;'a&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.Into.html\" title=\"trait core::convert::Into\">Into</a>&lt;<a class=\"enum\" href=\"quaint/ast/enum.DatabaseValue.html\" title=\"enum quaint::ast::DatabaseValue\">DatabaseValue</a>&lt;'a&gt;&gt;,&nbsp;</span>",synthetic:false,types:["quaint::ast::values::DatabaseValue"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"quaint/connector/struct.ResultSet.html\" title=\"struct quaint::connector::ResultSet\">ResultSet</a>&gt; for <a class=\"enum\" href=\"https://docs.rs/serde_json/1.0.41/serde_json/value/enum.Value.html\" title=\"enum serde_json::value::Value\">Value</a>",synthetic:false,types:["serde_json::value::Value"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;Error&gt; for <a class=\"enum\" href=\"quaint/error/enum.Error.html\" title=\"enum quaint::error::Error\">Error</a>",synthetic:false,types:["quaint::error::Error"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/alloc/string/struct.FromUtf8Error.html\" title=\"struct alloc::string::FromUtf8Error\">FromUtf8Error</a>&gt; for <a class=\"enum\" href=\"quaint/error/enum.Error.html\" title=\"enum quaint::error::Error\">Error</a>",synthetic:false,types:["quaint::error::Error"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://docs.rs/tokio-postgres/0.5/tokio_postgres/error/struct.Error.html\" title=\"struct tokio_postgres::error::Error\">Error</a>&gt; for <a class=\"enum\" href=\"quaint/error/enum.Error.html\" title=\"enum quaint::error::Error\">Error</a>",synthetic:false,types:["quaint::error::Error"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://docs.rs/native-tls/0.2/native_tls/struct.Error.html\" title=\"struct native_tls::Error\">Error</a>&gt; for <a class=\"enum\" href=\"quaint/error/enum.Error.html\" title=\"enum quaint::error::Error\">Error</a>",synthetic:false,types:["quaint::error::Error"]},{text:"impl&lt;'_&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;&amp;'_ <a class=\"struct\" href=\"https://docs.rs/native-tls/0.2/native_tls/struct.Error.html\" title=\"struct native_tls::Error\">Error</a>&gt; for <a class=\"enum\" href=\"quaint/error/enum.Error.html\" title=\"enum quaint::error::Error\">Error</a>",synthetic:false,types:["quaint::error::Error"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;Error&gt; for <a class=\"enum\" href=\"quaint/error/enum.Error.html\" title=\"enum quaint::error::Error\">Error</a>",synthetic:false,types:["quaint::error::Error"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;FromSqlError&gt; for <a class=\"enum\" href=\"quaint/error/enum.Error.html\" title=\"enum quaint::error::Error\">Error</a>",synthetic:false,types:["quaint::error::Error"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"https://docs.rs/url/1.7.0/url/parser/enum.ParseError.html\" title=\"enum url::parser::ParseError\">ParseError</a>&gt; for <a class=\"enum\" href=\"quaint/error/enum.Error.html\" title=\"enum quaint::error::Error\">Error</a>",synthetic:false,types:["quaint::error::Error"]},{text:"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/std/io/error/struct.Error.html\" title=\"struct std::io::error::Error\">Error</a>&gt; for <a class=\"enum\" href=\"quaint/error/enum.Error.html\" title=\"enum quaint::error::Error\">Error</a>",synthetic:false,types:["quaint::error::Error"]},{text:"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;<a class=\"enum\" href=\"quaint/ast/enum.ParameterizedValue.html\" title=\"enum quaint::ast::ParameterizedValue\">ParameterizedValue</a>&lt;'a&gt;&gt; for MyValue",synthetic:false,types:["mysql_common::value::Value"]},];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        })()