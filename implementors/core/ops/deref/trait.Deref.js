(function() {var implementors = {};
implementors["kernel_hal"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"kernel_hal/mem/struct.ZERO_FRAME.html\" title=\"struct kernel_hal::mem::ZERO_FRAME\">ZERO_FRAME</a>","synthetic":false,"types":["kernel_hal::common::mem::ZERO_FRAME"]},{"text":"impl&lt;P:&nbsp;<a class=\"trait\" href=\"kernel_hal/user/trait.Policy.html\" title=\"trait kernel_hal::user::Policy\">Policy</a>&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"kernel_hal/user/struct.IoVecs.html\" title=\"struct kernel_hal::user::IoVecs\">IoVecs</a>&lt;P&gt;","synthetic":false,"types":["kernel_hal::common::user::IoVecs"]}];
implementors["linux_object"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"linux_object/fs/struct.STDIN.html\" title=\"struct linux_object::fs::STDIN\">STDIN</a>","synthetic":false,"types":["linux_object::fs::stdio::STDIN"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"linux_object/fs/struct.STDOUT.html\" title=\"struct linux_object::fs::STDOUT\">STDOUT</a>","synthetic":false,"types":["linux_object::fs::stdio::STDOUT"]},{"text":"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"linux_object/sync/struct.SemaphoreGuard.html\" title=\"struct linux_object::sync::SemaphoreGuard\">SemaphoreGuard</a>&lt;'a&gt;","synthetic":false,"types":["linux_object::sync::semaphore::SemaphoreGuard"]}];
implementors["zcore_drivers"] = [{"text":"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"zcore_drivers/prelude/struct.FrameBuffer.html\" title=\"struct zcore_drivers::prelude::FrameBuffer\">FrameBuffer</a>&lt;'a&gt;","synthetic":false,"types":["zcore_drivers::scheme::display::FrameBuffer"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"zcore_drivers/utils/struct.GraphicConsole.html\" title=\"struct zcore_drivers::utils::GraphicConsole\">GraphicConsole</a>","synthetic":false,"types":["zcore_drivers::utils::graphic_console::GraphicConsole"]}];
implementors["zircon_object"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"zircon_object/task/struct.CurrentThread.html\" title=\"struct zircon_object::task::CurrentThread\">CurrentThread</a>","synthetic":false,"types":["zircon_object::task::thread::CurrentThread"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"zircon_object/vm/struct.VmObject.html\" title=\"struct zircon_object::vm::VmObject\">VmObject</a>","synthetic":false,"types":["zircon_object::vm::vmo::VmObject"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/deref/trait.Deref.html\" title=\"trait core::ops::deref::Deref\">Deref</a> for <a class=\"struct\" href=\"zircon_object/vm/struct.KERNEL_ASPACE.html\" title=\"struct zircon_object::vm::KERNEL_ASPACE\">KERNEL_ASPACE</a>","synthetic":false,"types":["zircon_object::vm::KERNEL_ASPACE"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()