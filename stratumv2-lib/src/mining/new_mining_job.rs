impl_message!(
    /// NewMiningJob is message sent from the Server -> Client to provide an
    /// updated mining job through a standard channel.
    NewMiningJob,

    /// The identifier of the standard channel.
    channel_id u32,

    /// A sequence of bytes that identifies the node to the Server, e.g.
    /// "braiintest.worker1".
    job_id STR0_255,

    /// The expected [h/s] (hash rate/per second) of the
    /// device or the cumulative on the channel if multiple devices are connected
    /// downstream. Proxies MUST send 0.0f when there are no mining devices
    /// connected yet.
    future_job f32,

    /// The Maximum Target that can be acceptd by the connected device or
    /// multiple devices downstream. The Server MUST accept the maximum
    /// target or respond by sending a
    /// [OpenStandardMiningChannel.Error](struct.OpenStandardMiningChannelError.html)
    /// or [OpenExtendedMiningChannel.Error](struct.OpenExtendedMiningChannelError.html)
    version U256,

    /// The minimum size of extranonce space required by the Downstream node.
    merkle_root u16
);
