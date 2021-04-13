mod mem_profiler;

use auxtools::*;

#[hook("/proc/mem_profiler_begin")]
fn begin(path: Value) {
    mem_profiler::begin(&path.as_string()?).map_err(|x| {
        runtime!("{:?}", x)
    })?;

    Ok(Value::null())
}

#[hook("/proc/mem_profiler_end")]
fn end() {
    mem_profiler::end();

    Ok(Value::null())
}
