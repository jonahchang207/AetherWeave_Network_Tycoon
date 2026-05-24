using System;
using System.Runtime.InteropServices;

namespace AetherWeave.Simulation
{
    /// <summary>
    /// P/Invoke surface for the Rust aetherweave-routing plugin.
    /// Implement FFI in native/aetherweave-routing/src/ffi.rs when the ABI stabilizes.
    /// </summary>
    public static class NativeRouting
    {
        const string LibName = "aetherweave_routing";

        // Placeholder — enable when ffi.rs exports are added:
        // [DllImport(LibName)] public static extern uint aw_graph_create();
        // [DllImport(LibName)] public static extern void aw_graph_destroy(uint handle);

        public static bool IsPluginLoaded
        {
            get
            {
                try
                {
                    // Probe once real exports exist.
                    return false;
                }
                catch (DllNotFoundException)
                {
                    return false;
                }
            }
        }
    }
}
