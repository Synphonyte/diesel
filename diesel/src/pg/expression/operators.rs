use crate::expression::expression_types::NotSelectable;
use crate::pg::Pg;
use crate::sql_types::{Bigint, Bool, Inet, Jsonb};

__diesel_infix_operator!(IsDistinctFrom, " IS DISTINCT FROM ", ConstantNullability Bool, backend: Pg);
__diesel_infix_operator!(IsNotDistinctFrom, " IS NOT DISTINCT FROM ", ConstantNullability Bool, backend: Pg);
infix_operator!(OverlapsWith, " && ", backend: Pg);
infix_operator!(Contains, " @> ", backend: Pg);
infix_operator!(IsContainedBy, " <@ ", backend: Pg);
infix_operator!(ILike, " ILIKE ", backend: Pg);
infix_operator!(NotILike, " NOT ILIKE ", backend: Pg);
infix_operator!(SimilarTo, " SIMILAR TO ", backend: Pg);
infix_operator!(NotSimilarTo, " NOT SIMILAR TO ", backend: Pg);
postfix_operator!(NullsFirst, " NULLS FIRST", NotSelectable, backend: Pg);
postfix_operator!(NullsLast, " NULLS LAST", NotSelectable, backend: Pg);
infix_operator!(ContainsNet, " >> ", backend: Pg);
infix_operator!(ContainsNetLoose, " >>= ", backend: Pg);
infix_operator!(IsContainedByNet, " << ", backend: Pg);
infix_operator!(IsContainedByNetLoose, " <<= ", backend: Pg);
infix_operator!(AndNet, " & ", Inet, backend: Pg);
infix_operator!(OrNet, " | ", Inet, backend: Pg);
infix_operator!(DifferenceNet, " - ", Bigint, backend: Pg);
infix_operator!(ConcatJsonb, " || ", Jsonb, backend: Pg);
infix_operator!(HasKeyJsonb, " ? ", backend: Pg);
infix_operator!(HasAnyKeyJsonb, " ?| ", backend: Pg);
infix_operator!(HasAllKeysJsonb, " ?& ", backend: Pg);
infix_operator!(ContainsJsonb, " @> ", backend: Pg);
infix_operator!(IsContainedByJsonb, " <@ ", backend: Pg);
