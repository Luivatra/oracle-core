use ergo_lib::ergotree_ir::chain::address::{AddressEncoder, NetworkAddress, NetworkPrefix};

use crate::contracts::{
    ballot::BallotContractParameters, oracle::OracleContractParameters,
    pool::PoolContractParameters, refresh::RefreshContractParameters,
    update::UpdateContractParameters,
};

pub trait DefaultWithNetworkPrefix {
    fn default_with(network_prefix: NetworkPrefix) -> Self;
}

impl DefaultWithNetworkPrefix for BallotContractParameters {
    fn default_with(network_prefix: NetworkPrefix) -> Self {
        // NOTE: slightly modified v2.0a from https://github.com/scalahub/OraclePool/blob/v2/src/main/scala/oraclepool/v2a/Contracts.scala
        // compiled via
        // https://wallet.plutomonkey.com/p2s/?source=eyAvLyBUaGlzIGJveCAoYmFsbG90IGJveCk6CiAgLy8gUjQgdGhlIGdyb3VwIGVsZW1lbnQgb2YgdGhlIG93bmVyIG9mIHRoZSBiYWxsb3QgdG9rZW4gW0dyb3VwRWxlbWVudF0KICAvLyBSNSB0aGUgY3JlYXRpb24gaGVpZ2h0IG9mIHRoZSB1cGRhdGUgYm94IFtJbnRdCiAgLy8gUjYgdGhlIHZhbHVlIHZvdGVkIGZvciBbQ29sbFtCeXRlXV0KICAvLyBSNyB0aGUgcmV3YXJkIHRva2VuIGlkIFtDb2xsW0J5dGVdXQogIC8vIFI4IHRoZSByZXdhcmQgdG9rZW4gYW1vdW50IFtMb25nXQoKICB2YWwgdXBkYXRlTkZUID0gZnJvbUJhc2U2NCgiWWxGbFZHaFhiVnB4TkhRM2R5RjZKVU1xUmkxS1FFNWpVbVpWYWxodU1uST0iKSAvLyBUT0RPIHJlcGxhY2Ugd2l0aCBhY3R1YWwgCiAgdmFsIG1pblN0b3JhZ2VSZW50ID0gMTAwMDAwMDBMICAvLyBUT0RPIHJlcGxhY2Ugd2l0aCBhY3R1YWwgIAogIHZhbCBzZWxmUHViS2V5ID0gU0VMRi5SNFtHcm91cEVsZW1lbnRdLmdldAogIHZhbCBvdXRJbmRleCA9IGdldFZhcltJbnRdKDApLmdldAogIHZhbCBvdXRwdXQgPSBPVVRQVVRTKG91dEluZGV4KQogIAogIHZhbCBpc1NpbXBsZUNvcHkgPSBvdXRwdXQuUjRbR3JvdXBFbGVtZW50XS5pc0RlZmluZWQgICAgICAgICAgICAgICAgJiYgLy8gYmFsbG90IGJveGVzIGFyZSB0cmFuc2ZlcmFibGUgYnkgc2V0dGluZyBkaWZmZXJlbnQgdmFsdWUgaGVyZSAKICAgICAgICAgICAgICAgICAgICAgb3V0cHV0LnByb3Bvc2l0aW9uQnl0ZXMgPT0gU0VMRi5wcm9wb3NpdGlvbkJ5dGVzICYmCiAgICAgICAgICAgICAgICAgICAgIG91dHB1dC50b2tlbnMgPT0gU0VMRi50b2tlbnMgICAgICAgICAgICAgICAgICAgICAmJiAKICAgICAgICAgICAgICAgICAgICAgb3V0cHV0LnZhbHVlID49IG1pblN0b3JhZ2VSZW50IAogIAogIHZhbCB1cGRhdGUgPSBJTlBVVFMuc2l6ZSA+IDEgICAgICAgICAgICAgICAgICAgICAgICAgICAmJgogICAgICAgICAgICAgICBJTlBVVFMoMSkudG9rZW5zLnNpemUgPiAwICAgICAgICAgICAgICAgICAmJgogICAgICAgICAgICAgICBJTlBVVFMoMSkudG9rZW5zKDApLl8xID09IHVwZGF0ZU5GVCAgICAgICAmJiAvLyBjYW4gb25seSB1cGRhdGUgd2hlbiB1cGRhdGUgYm94IGlzIHRoZSAybmQgaW5wdXQKICAgICAgICAgICAgICAgb3V0cHV0LlI0W0dyb3VwRWxlbWVudF0uZ2V0ID09IHNlbGZQdWJLZXkgJiYgLy8gcHVibGljIGtleSBpcyBwcmVzZXJ2ZWQKICAgICAgICAgICAgICAgb3V0cHV0LnZhbHVlID49IFNFTEYudmFsdWUgICAgICAgICAgICAgICAgJiYgLy8gdmFsdWUgcHJlc2VydmVkIG9yIGluY3JlYXNlZAogICAgICAgICAgICAgICAhIChvdXRwdXQuUjVbQW55XS5pc0RlZmluZWQpICAgICAgICAgICAgICAgICAvLyBubyBtb3JlIHJlZ2lzdGVyczsgcHJldmVudHMgYm94IGZyb20gYmVpbmcgcmV1c2VkIGFzIGEgdmFsaWQgdm90ZSAKICAKICB2YWwgb3duZXIgPSBwcm92ZURsb2coc2VsZlB1YktleSkKICAKICAvLyB1bmxpa2UgaW4gY29sbGVjdGlvbiwgaGVyZSB3ZSBkb24ndCByZXF1aXJlIHNwZW5kZXIgdG8gYmUgb25lIG9mIHRoZSBiYWxsb3QgdG9rZW4gaG9sZGVycwogIGlzU2ltcGxlQ29weSAmJiAob3duZXIgfHwgdXBkYXRlKQp9
        let address = AddressEncoder::new(network_prefix).parse_address_from_str("3whMZZ6THhGiX1avBy7KxoSNJrBEEJDhufAoWXq2qMiP5gy4ny5sVaFNrhJMybFASG7VP4DLTs2Mij6rMCQqj1D7JzMjHoguPL3W9y5g7JkWuYqrcN6AWWenaCmHa6jTueTsLmjBZnibb7L5SNJjRv1U5K9J3oazVkkmy19X2jQ3fDQ6tES8NAU1dngivpSAbuihur2tQ7ENCWeWZHkK49sUkxbWHgKRxHFFB1rT79Fs2mBp").unwrap();
        BallotContractParameters {
            p2s: NetworkAddress::new(NetworkPrefix::Mainnet, &address),
            min_storage_rent_index: 0,
            min_storage_rent: 10000000,
            update_nft_index: 6,
        }
    }
}

impl DefaultWithNetworkPrefix for OracleContractParameters {
    fn default_with(network_prefix: NetworkPrefix) -> Self {
        // via
        // https://wallet.plutomonkey.com/p2s/?source=eyAvLyBUaGlzIGJveCAob3JhY2xlIGJveCkKICAvLyAgIFI0IHB1YmxpYyBrZXkgKEdyb3VwRWxlbWVudCkgCiAgLy8gICBSNSBlcG9jaCBjb3VudGVyIG9mIGN1cnJlbnQgZXBvY2ggKEludCkKICAvLyAgIFI2IGRhdGEgcG9pbnQgKExvbmcpIG9yIGVtcHR5CgogIC8vICAgdG9rZW5zKDApIG9yYWNsZSB0b2tlbiAob25lKQogIC8vICAgdG9rZW5zKDEpIHJld2FyZCB0b2tlbnMgY29sbGVjdGVkIChvbmUgb3IgbW9yZSkgCiAgLy8gICAKICAvLyAgIFdoZW4gcHVibGlzaGluZyBhIGRhdGFwb2ludCwgdGhlcmUgbXVzdCBiZSBhdCBsZWFzdCBvbmUgcmV3YXJkIHRva2VuIGF0IGluZGV4IDEgCiAgLy8gIAogIC8vICAgV2Ugd2lsbCBjb25uZWN0IHRoaXMgYm94IHRvIHBvb2wgTkZUIGluIGlucHV0ICMwIChhbmQgbm90IHRoZSByZWZyZXNoIE5GVCBpbiBpbnB1dCAjMSkKICAvLyAgIFRoaXMgd2F5LCB3ZSBjYW4gY29udGludWUgdG8gdXNlIHRoZSBzYW1lIGJveCBhZnRlciB1cGRhdGluZyBwb29sCiAgLy8gICBUaGlzICpjb3VsZCogYWxsb3cgdGhlIG9yYWNsZSBib3ggdG8gYmUgc3BlbnQgZHVyaW5nIGFuIHVwZGF0ZQogIC8vICAgSG93ZXZlciwgdGhpcyBpcyBub3QgYW4gaXNzdWUgYmVjYXVzZSB0aGUgdXBkYXRlIGNvbnRyYWN0IGVuc3VyZXMgdGhhdCB0b2tlbnMgYW5kIHJlZ2lzdGVycyAoZXhjZXB0IHNjcmlwdCkgb2YgdGhlIHBvb2wgYm94IGFyZSBwcmVzZXJ2ZWQKCiAgLy8gICBQcml2YXRlIGtleSBob2xkZXIgY2FuIGRvIGZvbGxvd2luZyB0aGluZ3M6CiAgLy8gICAgIDEuIENoYW5nZSBncm91cCBlbGVtZW50IChwdWJsaWMga2V5KSBzdG9yZWQgaW4gUjQKICAvLyAgICAgMi4gU3RvcmUgYW55IHZhbHVlIG9mIHR5cGUgaW4gb3IgZGVsZXRlIGFueSB2YWx1ZSBmcm9tIFI0IHRvIFI5IAogIC8vICAgICAzLiBTdG9yZSBhbnkgdG9rZW4gb3Igbm9uZSBhdCAybmQgaW5kZXggCgogIC8vICAgSW4gb3JkZXIgdG8gY29ubmVjdCB0aGlzIG9yYWNsZSBib3ggdG8gYSBkaWZmZXJlbnQgcmVmcmVzaE5GVCBhZnRlciBhbiB1cGRhdGUsIAogIC8vICAgdGhlIG9yYWNsZSBzaG91bGQga2VlcCBhdCBsZWFzdCBvbmUgbmV3IHJld2FyZCB0b2tlbiBhdCBpbmRleCAxIHdoZW4gcHVibGlzaGluZyBkYXRhLXBvaW50CiAgCiAgdmFsIHBvb2xORlQgPSBmcm9tQmFzZTY0KCJSeXRMWWxCbFUyaFdiVmx4TTNRMmR6bDZKRU1tUmlsS1FFMWpVV1pVYWxjPSIpIC8vIFRPRE8gcmVwbGFjZSB3aXRoIGFjdHVhbCAKICAKICB2YWwgb3RoZXJUb2tlbklkID0gSU5QVVRTKDApLnRva2VucygwKS5fMQogIAogIHZhbCBtaW5TdG9yYWdlUmVudCA9IDEwMDAwMDAwTAogIHZhbCBzZWxmUHViS2V5ID0gU0VMRi5SNFtHcm91cEVsZW1lbnRdLmdldAogIHZhbCBvdXRJbmRleCA9IGdldFZhcltJbnRdKDApLmdldAogIHZhbCBvdXRwdXQgPSBPVVRQVVRTKG91dEluZGV4KQogIAogIHZhbCBpc1NpbXBsZUNvcHkgPSBvdXRwdXQudG9rZW5zKDApID09IFNFTEYudG9rZW5zKDApICAgICAgICAgICAgICAgICYmIC8vIG9yYWNsZSB0b2tlbiBpcyBwcmVzZXJ2ZWQKICAgICAgICAgICAgICAgICAgICAgb3V0cHV0LnByb3Bvc2l0aW9uQnl0ZXMgPT0gU0VMRi5wcm9wb3NpdGlvbkJ5dGVzICAmJiAvLyBzY3JpcHQgcHJlc2VydmVkCiAgICAgICAgICAgICAgICAgICAgIG91dHB1dC5SNFtHcm91cEVsZW1lbnRdLmlzRGVmaW5lZCAgICAgICAgICAgICAgICAgJiYgLy8gb3V0cHV0IG11c3QgaGF2ZSBhIHB1YmxpYyBrZXkgKG5vdCBuZWNlc3NhcmlseSB0aGUgc2FtZSkKICAgICAgICAgICAgICAgICAgICAgb3V0cHV0LnZhbHVlID49IG1pblN0b3JhZ2VSZW50ICAgICAgICAgICAgICAgICAgICAgICAvLyBlbnN1cmUgc3VmZmljaWVudCBFcmdzIHRvIGVuc3VyZSBubyBnYXJiYWdlIGNvbGxlY3Rpb24KICAgICAgICAgICAgICAgICAgICAgCiAgdmFsIGNvbGxlY3Rpb24gPSBvdGhlclRva2VuSWQgPT0gcG9vbE5GVCAgICAgICAgICAgICAgICAgICAgJiYgLy8gZmlyc3QgaW5wdXQgbXVzdCBiZSBwb29sIGJveAogICAgICAgICAgICAgICAgICAgb3V0cHV0LnRva2VucygxKS5fMSA9PSBTRUxGLnRva2VucygxKS5fMSAgICYmIC8vIHJld2FyZCB0b2tlbklkIGlzIHByZXNlcnZlZCAob3JhY2xlIHNob3VsZCBlbnN1cmUgdGhpcyBjb250YWlucyBhIHJld2FyZCB0b2tlbikKICAgICAgICAgICAgICAgICAgIG91dHB1dC50b2tlbnMoMSkuXzIgPiBTRUxGLnRva2VucygxKS5fMiAgICAmJiAvLyBhdCBsZWFzdCBvbmUgcmV3YXJkIHRva2VuIG11c3QgYmUgYWRkZWQgCiAgICAgICAgICAgICAgICAgICBvdXRwdXQuUjRbR3JvdXBFbGVtZW50XS5nZXQgPT0gc2VsZlB1YktleSAgJiYgLy8gZm9yIGNvbGxlY3Rpb24gcHJlc2VydmUgcHVibGljIGtleQogICAgICAgICAgICAgICAgICAgb3V0cHV0LnZhbHVlID49IFNFTEYudmFsdWUgICAgICAgICAgICAgICAgICYmIC8vIG5hbm9FcmdzIHZhbHVlIHByZXNlcnZlZAogICAgICAgICAgICAgICAgICAgISAob3V0cHV0LlI1W0FueV0uaXNEZWZpbmVkKSAgICAgICAgICAgICAgICAgIC8vIG5vIG1vcmUgcmVnaXN0ZXJzOyBwcmV2ZW50cyBib3ggZnJvbSBiZWluZyByZXVzZWQgYXMgYSB2YWxpZCBkYXRhLXBvaW50CgogIHZhbCBvd25lciA9IHByb3ZlRGxvZyhzZWxmUHViS2V5KSAgCgogIC8vIG93bmVyIGNhbiBjaG9vc2UgdG8gdHJhbnNmZXIgdG8gYW5vdGhlciBwdWJsaWMga2V5IGJ5IHNldHRpbmcgZGlmZmVyZW50IHZhbHVlIGluIFI0CiAgaXNTaW1wbGVDb3B5ICYmIChvd25lciB8fCBjb2xsZWN0aW9uKSAKfQ==
        let address = AddressEncoder::new(network_prefix).parse_address_from_str("2vTHJzWVd7ryXrP3fH9KfEFGzS8XFdVY99xXuxMPt664HurrUn3e8y3W1wTQDVZsDi9TDeZdun2XEr3pcipGmKdmciSADmKn32Cs8YuPLNp4zaBZNo6m6NG8tz3zznb56nRCrz5VDDjxYTsQ92DqhtQmG3m7H6zbtNHLzJjf7x9ZSD3vNWRL6e7usRjfm1diob8bdizsbJM7wNDzLZYhshHScEkWse9MQKgMDN4pYb1vQLR1PmvUnpsRAjRYwNBs3ZjJoqdSpN6jbjfSJsrgEhBANbnCZxP3dKBr").unwrap();
        OracleContractParameters {
            p2s: NetworkAddress::new(NetworkPrefix::Mainnet, &address),
            pool_nft_index: 5,
        }
    }
}

impl DefaultWithNetworkPrefix for PoolContractParameters {
    fn default_with(network_prefix: NetworkPrefix) -> Self {
        // via
        // https://wallet.plutomonkey.com/p2s/?source=ewogIC8vIFRoaXMgYm94IChwb29sIGJveCkKICAvLyAgIGVwb2NoIHN0YXJ0IGhlaWdodCBpcyBzdG9yZWQgaW4gY3JlYXRpb24gSGVpZ2h0IChSMykKICAvLyAgIFI0IEN1cnJlbnQgZGF0YSBwb2ludCAoTG9uZykKICAvLyAgIFI1IEN1cnJlbnQgZXBvY2ggY291bnRlciAoSW50KQogIC8vCiAgLy8gICB0b2tlbnMoMCkgcG9vbCB0b2tlbiAoTkZUKQogIC8vICAgdG9rZW5zKDEpIHJld2FyZCB0b2tlbnMKICAvLyAgIFdoZW4gaW5pdGlhbGl6aW5nIHRoZSBib3gsIHRoZXJlIG11c3QgYmUgb25lIHJld2FyZCB0b2tlbi4gV2hlbiBjbGFpbWluZyByZXdhcmQsIG9uZSB0b2tlbiBtdXN0IGJlIGxlZnQgdW5jbGFpbWVkCiAgCiAgdmFsIG90aGVyVG9rZW5JZCA9IElOUFVUUygxKS50b2tlbnMoMCkuXzEKICB2YWwgcmVmcmVzaE5GVCA9IGZyb21CYXNlNjQoIlZHcFhibHB5TkhVM2VDRkJKVVFxUnkxTFlVNWtVbWRWYTFod01uTTFkamc9IikgLy8gVE9ETyByZXBsYWNlIHdpdGggYWN0dWFsCiAgdmFsIHVwZGF0ZU5GVCA9IGZyb21CYXNlNjQoIllsRmxWR2hYYlZweE5IUTNkeUY2SlVNcVJpMUtRRTVqVW1aVmFsaHVNbkk9IikgLy8gVE9ETyByZXBsYWNlIHdpdGggYWN0dWFsCgogIHNpZ21hUHJvcChvdGhlclRva2VuSWQgPT0gcmVmcmVzaE5GVCB8fCBvdGhlclRva2VuSWQgPT0gdXBkYXRlTkZUKQp9
        let address = AddressEncoder::new(network_prefix).parse_address_from_str("PViBL5acX6PoP6BQPsYtyNzW9aPXwxpRaUkXo4nE7RkxcBbZXJECUEBQm4g3MQCb2QsQALqPkrDN9TvsKuQkChF8sZSfnH5fifgKAkXhW8ifAcAE1qA67n9mabB3Mb2R8xT2v3SN49eN8mQ8HN95").unwrap();
        PoolContractParameters {
            p2s: NetworkAddress::new(NetworkPrefix::Mainnet, &address),
            refresh_nft_index: 2,
            update_nft_index: 3,
        }
    }
}

impl DefaultWithNetworkPrefix for RefreshContractParameters {
    fn default_with(network_prefix: NetworkPrefix) -> Self {
        // v2.0a from https://github.com/scalahub/OraclePool/blob/v2/src/main/scala/oraclepool/v2a/Contracts.scala
        // compiled via
        // https://wallet.plutomonkey.com/p2s/?source=eyAvLyBUaGlzIGJveCAocmVmcmVzaCBib3gpCiAgLy8gICB0b2tlbnMoMCkgcmV3YXJkIHRva2VucyB0byBiZSBlbWl0dGVkIChzZXZlcmFsKSAKICAvLyAgIAogIC8vICAgV2hlbiBpbml0aWFsaXppbmcgdGhlIGJveCwgdGhlcmUgbXVzdCBiZSBvbmUgcmV3YXJkIHRva2VuLiBXaGVuIGNsYWltaW5nIHJld2FyZCwgb25lIHRva2VuIG11c3QgYmUgbGVmdCB1bmNsYWltZWQgICAKICAKICB2YWwgb3JhY2xlVG9rZW5JZCA9IGZyb21CYXNlNjQoIktrY3RTbUZPWkZKblZXdFljREp6TlhZNGVTOUNQMFVvU0N0TllsQmxVMmc9IikgLy8gVE9ETyByZXBsYWNlIHdpdGggYWN0dWFsCiAgdmFsIHBvb2xORlQgPSBmcm9tQmFzZTY0KCJSeXRMWWxCbFUyaFdiVmx4TTNRMmR6bDZKRU1tUmlsS1FFMWpVV1pVYWxjPSIpIC8vIFRPRE8gcmVwbGFjZSB3aXRoIGFjdHVhbCAKICB2YWwgZXBvY2hMZW5ndGggPSAzMCAvLyBUT0RPIHJlcGxhY2Ugd2l0aCBhY3R1YWwKICB2YWwgbWluRGF0YVBvaW50cyA9IDQgLy8gVE9ETyByZXBsYWNlIHdpdGggYWN0dWFsCiAgdmFsIGJ1ZmZlciA9IDQgLy8gVE9ETyByZXBsYWNlIHdpdGggYWN0dWFsCiAgdmFsIG1heERldmlhdGlvblBlcmNlbnQgPSA1IC8vIHBlcmNlbnQgLy8gVE9ETyByZXBsYWNlIHdpdGggYWN0dWFsCgogIHZhbCBtaW5TdGFydEhlaWdodCA9IEhFSUdIVCAtIGVwb2NoTGVuZ3RoCiAgdmFsIHNwZW5kZXJJbmRleCA9IGdldFZhcltJbnRdKDApLmdldCAvLyB0aGUgaW5kZXggb2YgdGhlIGRhdGEtcG9pbnQgYm94IChOT1QgaW5wdXQhKSBiZWxvbmdpbmcgdG8gc3BlbmRlciAgICAKICAgIAogIHZhbCBwb29sSW4gPSBJTlBVVFMoMCkKICB2YWwgcG9vbE91dCA9IE9VVFBVVFMoMCkKICB2YWwgc2VsZk91dCA9IE9VVFBVVFMoMSkKCiAgZGVmIGlzVmFsaWREYXRhUG9pbnQoYjogQm94KSA9IGlmIChiLlI2W0xvbmddLmlzRGVmaW5lZCkgewogICAgYi5jcmVhdGlvbkluZm8uXzEgICAgPj0gbWluU3RhcnRIZWlnaHQgJiYgIC8vIGRhdGEgcG9pbnQgbXVzdCBub3QgYmUgdG9vIG9sZAogICAgYi50b2tlbnMoMCkuXzEgICAgICAgPT0gb3JhY2xlVG9rZW5JZCAgJiYgLy8gZmlyc3QgdG9rZW4gaWQgbXVzdCBiZSBvZiBvcmFjbGUgdG9rZW4KICAgIGIuUjVbSW50XS5nZXQgICAgICAgID09IHBvb2xJbi5SNVtJbnRdLmdldCAvLyBpdCBtdXN0IGNvcnJlc3BvbmQgdG8gdGhpcyBlcG9jaAogIH0gZWxzZSBmYWxzZSAKICAgICAgICAgIAogIHZhbCBkYXRhUG9pbnRzID0gSU5QVVRTLmZpbHRlcihpc1ZhbGlkRGF0YVBvaW50KSAgICAKICB2YWwgcHViS2V5ID0gZGF0YVBvaW50cyhzcGVuZGVySW5kZXgpLlI0W0dyb3VwRWxlbWVudF0uZ2V0CgogIHZhbCBlbm91Z2hEYXRhUG9pbnRzID0gZGF0YVBvaW50cy5zaXplID49IG1pbkRhdGFQb2ludHMgICAgCiAgdmFsIHJld2FyZEVtaXR0ZWQgPSBkYXRhUG9pbnRzLnNpemUgKiAyIC8vIG9uZSBleHRyYSB0b2tlbiBmb3IgZWFjaCBjb2xsZWN0ZWQgYm94IGFzIHJld2FyZCB0byBjb2xsZWN0b3IgICAKICB2YWwgZXBvY2hPdmVyID0gcG9vbEluLmNyZWF0aW9uSW5mby5fMSA8IG1pblN0YXJ0SGVpZ2h0CiAgICAgICAKICB2YWwgc3RhcnREYXRhID0gMUwgLy8gd2UgZG9uJ3QgYWxsb3cgMCBkYXRhIHBvaW50cwogIHZhbCBzdGFydFN1bSA9IDBMIAogIC8vIHdlIGV4cGVjdCBkYXRhLXBvaW50cyB0byBiZSBzb3J0ZWQgaW4gSU5DUkVBU0lORyBvcmRlcgogIAogIHZhbCBsYXN0U29ydGVkU3VtID0gZGF0YVBvaW50cy5mb2xkKChzdGFydERhdGEsICh0cnVlLCBzdGFydFN1bSkpLCB7CiAgICAgICAgKHQ6IChMb25nLCAoQm9vbGVhbiwgTG9uZykpLCBiOiBCb3gpID0+CiAgICAgICAgICAgdmFsIGN1cnJEYXRhID0gYi5SNltMb25nXS5nZXQKICAgICAgICAgICB2YWwgcHJldkRhdGEgPSB0Ll8xCiAgICAgICAgICAgdmFsIHdhc1NvcnRlZCA9IHQuXzIuXzEgCiAgICAgICAgICAgdmFsIG9sZFN1bSA9IHQuXzIuXzIKICAgICAgICAgICB2YWwgbmV3U3VtID0gb2xkU3VtICsgY3VyckRhdGEgIC8vIHdlIGRvbid0IGhhdmUgdG8gd29ycnkgYWJvdXQgb3ZlcmZsb3csIGFzIGl0IGNhdXNlcyBzY3JpcHQgdG8gZmFpbAoKICAgICAgICAgICB2YWwgaXNTb3J0ZWQgPSB3YXNTb3J0ZWQgJiYgcHJldkRhdGEgPD0gY3VyckRhdGEgCgogICAgICAgICAgIChjdXJyRGF0YSwgKGlzU29ydGVkLCBuZXdTdW0pKQogICAgfQogICkKIAogIHZhbCBsYXN0RGF0YSA9IGxhc3RTb3J0ZWRTdW0uXzEKICB2YWwgaXNTb3J0ZWQgPSBsYXN0U29ydGVkU3VtLl8yLl8xCiAgdmFsIHN1bSA9IGxhc3RTb3J0ZWRTdW0uXzIuXzIKICB2YWwgYXZlcmFnZSA9IHN1bSAvIGRhdGFQb2ludHMuc2l6ZSAKCiAgdmFsIG1heERlbHRhID0gbGFzdERhdGEgKiBtYXhEZXZpYXRpb25QZXJjZW50IC8gMTAwICAgICAgICAgIAogIHZhbCBmaXJzdERhdGEgPSBkYXRhUG9pbnRzKDApLlI2W0xvbmddLmdldAoKICBwcm92ZURsb2cocHViS2V5KSAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICYmCiAgZXBvY2hPdmVyICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAmJiAKICBlbm91Z2hEYXRhUG9pbnRzICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICYmICAgIAogIGlzU29ydGVkICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgJiYKICBsYXN0RGF0YSAtIGZpcnN0RGF0YSAgICAgPD0gbWF4RGVsdGEgICAgICAgICAgICAgICAgICAgICAgICAgICYmIAogIHBvb2xJbi50b2tlbnMoMCkuXzEgICAgICA9PSBwb29sTkZUICAgICAgICAgICAgICAgICAgICAgICAgICAgJiYKICBwb29sT3V0LnRva2VucygwKSAgICAgICAgPT0gcG9vbEluLnRva2VucygwKSAgICAgICAgICAgICAgICAgICYmIC8vIHByZXNlcnZlIHBvb2wgTkZUCiAgcG9vbE91dC50b2tlbnMoMSkuXzEgICAgID09IHBvb2xJbi50b2tlbnMoMSkuXzEgICAgICAgICAgICAgICAmJiAvLyByZXdhcmQgdG9rZW4gaWQgcHJlc2VydmVkCiAgcG9vbE91dC50b2tlbnMoMSkuXzIgICAgID49IHBvb2xJbi50b2tlbnMoMSkuXzIgLSByZXdhcmRFbWl0dGVkICYmIC8vIHJld2FyZCB0b2tlbiBhbW91bnQgY29ycmVjdGx5IHJlZHVjZWQKICBwb29sT3V0LnRva2Vucy5zaXplICAgICAgICA9PSBwb29sSW4udG9rZW5zLnNpemUgICAgICAgICAgICAgICYmIC8vIGNhbm5vdCBpbmplY3QgbW9yZSB0b2tlbnMgdG8gcG9vbCBib3gKICBwb29sT3V0LlI0W0xvbmddLmdldCAgICAgPT0gYXZlcmFnZSAgICAgICAgICAgICAgICAgICAgICAgICAgICYmIC8vIHJhdGUKICBwb29sT3V0LlI1W0ludF0uZ2V0ICAgICAgPT0gcG9vbEluLlI1W0ludF0uZ2V0ICsgMSAgICAgICAgICAgICYmIC8vIGNvdW50ZXIKICBwb29sT3V0LnByb3Bvc2l0aW9uQnl0ZXMgPT0gcG9vbEluLnByb3Bvc2l0aW9uQnl0ZXMgICAgICAgICAgICYmIC8vIHByZXNlcnZlIHBvb2wgc2NyaXB0CiAgcG9vbE91dC52YWx1ZSAgICAgICAgICAgID49IHBvb2xJbi52YWx1ZSAgICAgICAgICAgICAgICAgICAgICAmJgogIHBvb2xPdXQuY3JlYXRpb25JbmZvLl8xICA+PSBIRUlHSFQgLSBidWZmZXIgICAgICAgICAgICAgICAgICAgJiYgLy8gZW5zdXJlIHRoYXQgbmV3IGJveCBoYXMgY29ycmVjdCBzdGFydCBlcG9jaCBoZWlnaHQKICBzZWxmT3V0LnRva2VucyAgICAgICAgICAgPT0gU0VMRi50b2tlbnMgICAgICAgICAgICAgICAgICAgICAgICYmIC8vIHJlZnJlc2ggTkZUIHByZXNlcnZlZAogIHNlbGZPdXQucHJvcG9zaXRpb25CeXRlcyA9PSBTRUxGLnByb3Bvc2l0aW9uQnl0ZXMgICAgICAgICAgICAgJiYgLy8gc2NyaXB0IHByZXNlcnZlZAogIHNlbGZPdXQudmFsdWUgICAgICAgICAgICA+PSBTRUxGLnZhbHVlICAgICAgICAgICAgICAgICAgICAgICAKfQ==_
        let address = AddressEncoder::new(network_prefix).parse_address_from_str("oq3jWGvabYxVYtceq1RGzFD4UdcdHcqY861G7H4mDiEnYQHya17A2w5r7u45moTpjAqfsNTm2XyhRNvYHiZhDTpmnfVa9XHSsbs5zjEw5UmgQfuP5d3NdFVy7oiAvLP1sjZN8qiHryzFoenLgtsxV8wLAeBaRChy73dd3rgyVfZipVL5LCXQyXMqp9oFFzPtTPkBw3ha7gJ4Bs5KjeUkVXJRVQ2Tdhg51Sdb6fEkHRtRuvCpynxYokQXP6SNif1M6mPcBR3B4zMLcFvmGxwNkZ3mRFzqHVzHV8Syu5AzueJEmMTrvWAXnhpYE7WcFbmDt3dqyXq7x9DNyKq1VwRwgFscLYDenAHqqHKd3jsJ6Grs8uFvvvJGKdqzdoJ3qCcCRXeDcZAKmExJMH4hJbsk8b1ct5YDBcNrq3LUr319XkS8miZDbHdHa88MSpCJQJmE51hmWVAV1yXrpyxqXqAXXPpSaGCP38BwCv8hYFK37DyA4mQd5r7vF9vNo5DEXwQ5wA2EivwRtNqpKUxXtKuZWTNC7Pu7NmvEHSuJPnaoCUujCiPtLM4dR64u8Gp7X3Ujo3o9zuMc6npemx3hf8rQS18QXgKJLwfeSqVYkicbVcGZRHsPsGxwrf1Wixp45E8d5e97MsKTCuqSskPKaHUdQYW1JZ8djcr4dxg1qQN81m7u2q8dwW6AK32mwRSS3nj27jkjML6n6GBpNZk9AtB2uMx3CHo6pZSaxgeCXuu3amrdeYmbuSqHUNZHU").unwrap();
        RefreshContractParameters {
            p2s: NetworkAddress::new(NetworkPrefix::Mainnet, &address),
            pool_nft_index: 17,
            oracle_token_id_index: 3,
            min_data_points_index: 13,
            min_data_points: 4,
            buffer_index: 21,
            buffer_length: 4,
            max_deviation_percent_index: 15,
            max_deviation_percent: 5,
            epoch_length_index: 0,
            epoch_length: 30,
        }
    }
}

impl DefaultWithNetworkPrefix for UpdateContractParameters {
    fn default_with(network_prefix: NetworkPrefix) -> Self {
        // from https://wallet.plutomonkey.com/p2s/?source=eyAvLyBUaGlzIGJveCAodXBkYXRlIGJveCk6CiAgICAgICAgIC8vIFJlZ2lzdGVycyBlbXB0eSAKICAgICAgICAgLy8gCiAgICAgICAgIC8vIGJhbGxvdCBib3hlcyAoSW5wdXRzKQogICAgICAgICAvLyBSNCB0aGUgcHViIGtleSBvZiB2b3RlciBbR3JvdXBFbGVtZW50XSAobm90IHVzZWQgaGVyZSkKICAgICAgICAgLy8gUjUgdGhlIGNyZWF0aW9uIGhlaWdodCBvZiB0aGlzIGJveCBbSW50XQogICAgICAgICAvLyBSNiB0aGUgdmFsdWUgdm90ZWQgZm9yIFtDb2xsW0J5dGVdXSAoaGFzaCBvZiB0aGUgbmV3IHBvb2wgYm94IHNjcmlwdCkKICAgICAgICAgLy8gUjcgdGhlIHJld2FyZCB0b2tlbiBpZCBpbiBuZXcgYm94IAogICAgICAgICAvLyBSOCB0aGUgbnVtYmVyIG9mIHJld2FyZCB0b2tlbnMgaW4gbmV3IGJveCAKICAgICAgIAogICAgICAgICB2YWwgcG9vbE5GVCA9IGZyb21CYXNlNjQoIlJ5dExZbEJsVTJoV2JWbHhNM1EyZHpsNkpFTW1SaWxLUUUxalVXWlVhbGM9IikgLy8gVE9ETyByZXBsYWNlIHdpdGggYWN0dWFsIAogICAgICAKICAgICAgICAgdmFsIGJhbGxvdFRva2VuSWQgPSBmcm9tQmFzZTY0KCJQMFFvUnkxTFlWQmtVMmRXYTFsd00zTTJkamw1SkVJbVJTbElRRTFpVVdVPSIpIC8vIFRPRE8gcmVwbGFjZSB3aXRoIGFjdHVhbCAKICAgICAgIAogICAgICAgICB2YWwgbWluVm90ZXMgPSA2IC8vIFRPRE8gcmVwbGFjZSB3aXRoIGFjdHVhbAogICAgICAgICAKICAgICAgICAgdmFsIHBvb2xJbiA9IElOUFVUUygwKSAvLyBwb29sIGJveCBpcyAxc3QgaW5wdXQKICAgICAgICAgdmFsIHBvb2xPdXQgPSBPVVRQVVRTKDApIC8vIGNvcHkgb2YgcG9vbCBib3ggaXMgdGhlIDFzdCBvdXRwdXQKICAgICAgIAogICAgICAgICB2YWwgdXBkYXRlQm94T3V0ID0gT1VUUFVUUygxKSAvLyBjb3B5IG9mIHRoaXMgYm94IGlzIHRoZSAybmQgb3V0cHV0CiAgICAgICAKICAgICAgICAgLy8gY29tcHV0ZSB0aGUgaGFzaCBvZiB0aGUgcG9vbCBvdXRwdXQgYm94LiBUaGlzIHNob3VsZCBiZSB0aGUgdmFsdWUgdm90ZWQgZm9yCiAgICAgICAgIHZhbCBwb29sT3V0SGFzaCA9IGJsYWtlMmIyNTYocG9vbE91dC5wcm9wb3NpdGlvbkJ5dGVzKQogICAgICAgICB2YWwgcmV3YXJkVG9rZW5JZCA9IHBvb2xPdXQudG9rZW5zKDEpLl8xCiAgICAgICAgIHZhbCByZXdhcmRBbXQgPSBwb29sT3V0LnRva2VucygxKS5fMgogICAgICAgICAKICAgICAgICAgdmFsIHZhbGlkUG9vbEluID0gcG9vbEluLnRva2VucygwKS5fMSA9PSBwb29sTkZUCiAgICAgICAgIAogICAgICAgICB2YWwgdmFsaWRQb29sT3V0ID0gcG9vbEluLnByb3Bvc2l0aW9uQnl0ZXMgIT0gcG9vbE91dC5wcm9wb3NpdGlvbkJ5dGVzICAmJiAvLyBzY3JpcHQgc2hvdWxkIG5vdCBiZSBwcmVzZXJ2ZWQKICAgICAgICAgICAgICAgICAgICAgICAgICAgIHBvb2xJbi50b2tlbnMoMCkgPT0gcG9vbE91dC50b2tlbnMoMCkgICAgICAgICAgICAgICAgJiYgLy8gTkZUIHByZXNlcnZlZAogICAgICAgICAgICAgICAgICAgICAgICAgICAgcG9vbEluLmNyZWF0aW9uSW5mby5fMSA9PSBwb29sT3V0LmNyZWF0aW9uSW5mby5fMSAgICAmJiAvLyBjcmVhdGlvbiBoZWlnaHQgcHJlc2VydmVkCiAgICAgICAgICAgICAgICAgICAgICAgICAgICBwb29sSW4udmFsdWUgPT0gcG9vbE91dC52YWx1ZSAgICAgICAgICAgICAgICAgICAgICAgICYmIC8vIHZhbHVlIHByZXNlcnZlZCAKICAgICAgICAgICAgICAgICAgICAgICAgICAgIHBvb2xJbi5SNFtMb25nXSA9PSBwb29sT3V0LlI0W0xvbmddICAgICAgICAgICAgICAgICAgJiYgLy8gcmF0ZSBwcmVzZXJ2ZWQgIAogICAgICAgICAgICAgICAgICAgICAgICAgICAgcG9vbEluLlI1W0ludF0gPT0gcG9vbE91dC5SNVtJbnRdICAgICAgICAgICAgICAgICAgICAmJiAvLyBjb3VudGVyIHByZXNlcnZlZAogICAgICAgICAgICAgICAgICAgICAgICAgICAgISAocG9vbE91dC5SNltBbnldLmlzRGVmaW5lZCkKICAgICAgIAogICAgICAgICAKICAgICAgICAgdmFsIHZhbGlkVXBkYXRlT3V0ID0gdXBkYXRlQm94T3V0LnRva2VucyA9PSBTRUxGLnRva2VucyAgICAgICAgICAgICAgICAgICAgICYmCiAgICAgICAgICAgICAgICAgICAgICAgICAgICAgIHVwZGF0ZUJveE91dC5wcm9wb3NpdGlvbkJ5dGVzID09IFNFTEYucHJvcG9zaXRpb25CeXRlcyAmJgogICAgICAgICAgICAgICAgICAgICAgICAgICAgICB1cGRhdGVCb3hPdXQudmFsdWUgPj0gU0VMRi52YWx1ZSAgICAgICAgICAgICAgICAgICAgICAgJiYKICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgdXBkYXRlQm94T3V0LmNyZWF0aW9uSW5mby5fMSA+IFNFTEYuY3JlYXRpb25JbmZvLl8xICAgICYmCiAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICEgKHVwZGF0ZUJveE91dC5SNFtBbnldLmlzRGVmaW5lZCkgCiAgICAgICAKICAgICAgICAgZGVmIGlzVmFsaWRCYWxsb3QoYjpCb3gpID0gaWYgKGIudG9rZW5zLnNpemUgPiAwKSB7CiAgICAgICAgICAgYi50b2tlbnMoMCkuXzEgPT0gYmFsbG90VG9rZW5JZCAgICAgICAmJgogICAgICAgICAgIGIuUjVbSW50XS5nZXQgPT0gU0VMRi5jcmVhdGlvbkluZm8uXzEgJiYgLy8gZW5zdXJlIHZvdGUgY29ycmVzcG9uZHMgdG8gdGhpcyBib3ggYnkgY2hlY2tpbmcgY3JlYXRpb24gaGVpZ2h0CiAgICAgICAgICAgYi5SNltDb2xsW0J5dGVdXS5nZXQgPT0gcG9vbE91dEhhc2ggICAmJiAvLyBjaGVjayBwcm9wb3NpdGlvbiB2b3RlZCBmb3IKICAgICAgICAgICBiLlI3W0NvbGxbQnl0ZV1dLmdldCA9PSByZXdhcmRUb2tlbklkICYmIC8vIGNoZWNrIHJld2FyZFRva2VuSWQgdm90ZWQgZm9yCiAgICAgICAgICAgYi5SOFtMb25nXS5nZXQgPT0gcmV3YXJkQW10ICAgICAgICAgICAgICAvLyBjaGVjayByZXdhcmRUb2tlbkFtdCB2b3RlZCBmb3IKICAgICAgICAgfSBlbHNlIGZhbHNlCiAgICAgICAgIAogICAgICAgICB2YWwgYmFsbG90Qm94ZXMgPSBJTlBVVFMuZmlsdGVyKGlzVmFsaWRCYWxsb3QpCiAgICAgICAgIAogICAgICAgICB2YWwgdm90ZXNDb3VudCA9IGJhbGxvdEJveGVzLmZvbGQoMEwsIHsoYWNjdW06IExvbmcsIGI6IEJveCkgPT4gYWNjdW0gKyBiLnRva2VucygwKS5fMn0pCiAgICAgICAgIAogICAgICAgICBzaWdtYVByb3AodmFsaWRQb29sSW4gJiYgdmFsaWRQb29sT3V0ICYmIHZhbGlkVXBkYXRlT3V0ICYmIHZvdGVzQ291bnQgPj0gbWluVm90ZXMpICAKICAgICAgIH0KICAgICAgIA==
        let address = AddressEncoder::new(network_prefix).parse_address_from_str("RGQjcwtwcPBVwTFZMaGyo471kgwcwtMjrUy41RqWhAtY2ovdKAQ2Ce3cUaF6S7LGMrV3boM5rGKR5K2vjyheDXtVuEoUpZefQ2qa7H8MPBaYfAWqttNpyp5A1GfYviWfSbbEsbUSptgUMHH9MTLCnkvQdfxtC9HvKX8gJdaJBhEF4KHUBDVcsuMX33vcqi7Y5voEjunnmgvbpcYBG6HAkZtz15uXh1TskFpumFDgqwMbExapeRRXbq3EjuVAqEeoibastYMLrZ1evAq1bZP9mFoQRd15kUgBHvRQLwHJzdcRSz1pCM6UXTsna599VQBCiqKRZ9iCDffeUGuvjJBgzm5gouMCpaEc6LJn5Z2ta5MFAvQpd1MhtvTBL6X6NFKbYJxNWFK7igqbf9nDtbkcrUjRD2LKeqEapNRbLnxyMd6Dd5nMKZLuthkgsK3BSmN4YKh2S94wNE5PRDM1FULTg1RC7tFvRV5aKmcKD25M7qYwXwLqWoRPCk7C8CqCdSHT2cJTM3RAx6xSbt5Cq").unwrap();
        UpdateContractParameters {
            p2s: NetworkAddress::new(NetworkPrefix::Mainnet, &address),
            pool_nft_index: 5,
            ballot_token_index: 9,
            min_votes_index: 13,
            min_votes: 6,
        }
    }
}
