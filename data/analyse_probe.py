"""Per-rule exponents from probe windows, fitted over k in [KLO, KHI]."""
import sys
import numpy as np
import polars as pl

KLO, KHI = 20, 26


def slope(x, y):
    return float(np.polyfit(np.log(x), np.log(y), 1)[0])


def per_rule(df: pl.DataFrame) -> pl.DataFrame:
    df = df.filter(~pl.col("escaped")).with_columns(
        steps=pl.col("t") // 2,
        width_max=pl.max_horizontal("width", "height"),
    ).with_columns(
        c=pl.col("new_cells") / pl.col("boundary_steps"),
        B=pl.col("boundary_steps") / pl.col("steps"),
        PA=pl.col("perimeter") / pl.col("area"),
        rate=pl.col("new_cells") / pl.col("steps"),
    ).with_columns(
        kappa=pl.col("B") / pl.col("PA"),
        fill=pl.col("area") / (pl.col("width") * pl.col("height")),
        spread=pl.col("ant_r2") / pl.col("rg2"),
    )
    out = []
    for (rule,), g in df.group_by("rule"):
        g = g.filter((pl.col("k") >= KLO) & (pl.col("k") <= KHI)).sort("k")
        if g.height < KHI - KLO + 1 or (g["new_cells"] == 0).any():
            continue
        t = g["t"].to_numpy().astype(float)
        a = slope(t, g["area"].to_numpy())
        out.append(dict(
            rule=rule, n=len(rule),
            alpha_w=slope(t, g["width_max"].to_numpy()),
            alpha_rg=slope(t, np.sqrt(g["rg2"].to_numpy())) ,
            a=a,
            beta=slope(g["area"].to_numpy(), g["perimeter"].to_numpy()),
            g_c=slope(t, g["c"].to_numpy()),
            g_k=slope(t, g["kappa"].to_numpy()),
            c=float(g["c"][-1]), kappa=float(g["kappa"][-1]),
            fill=float(g["fill"][-1]), spread=float(g["spread"][-1]),
            g_fill=slope(t, g["fill"].to_numpy()),
        ))
    r = pl.DataFrame(out)
    # Prediction from the mechanism: a = (1 + g_c + g_k) / (2 - beta).
    return r.with_columns(a_pred=(1 + pl.col("g_c") + pl.col("g_k")) / (2 - pl.col("beta")))


if __name__ == "__main__":
    r = per_rule(pl.read_csv(sys.argv[1]))
    pl.Config.set_tbl_rows(40); pl.Config.set_tbl_cols(20); pl.Config.set_float_precision(3)
    print(r.sort("alpha_w").select("rule", "alpha_w", "alpha_rg", "a", "a_pred", "beta", "g_c", "g_k", "c", "kappa", "fill", "spread"))
