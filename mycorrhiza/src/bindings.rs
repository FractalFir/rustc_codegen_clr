
    pub mod Microsoft{
    pub mod Win32{
    pub mod SafeHandles{
    pub type CriticalHandleMinusOneIsInvalid =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","Microsoft.Win32.SafeHandles.CriticalHandleMinusOneIsInvalid">;
    use super::super::super::*;
    impl From<CriticalHandleMinusOneIsInvalid> for System::Runtime::InteropServices::CriticalHandle {
     fn from(v:CriticalHandleMinusOneIsInvalid)->System::Runtime::InteropServices::CriticalHandle{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Runtime::InteropServices::CriticalHandle,CriticalHandleMinusOneIsInvalid>(v)
    }} 
    pub type CriticalHandleZeroOrMinusOneIsInvalid =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","Microsoft.Win32.SafeHandles.CriticalHandleZeroOrMinusOneIsInvalid">;
    use super::super::super::*;
    impl From<CriticalHandleZeroOrMinusOneIsInvalid> for System::Runtime::InteropServices::CriticalHandle {
     fn from(v:CriticalHandleZeroOrMinusOneIsInvalid)->System::Runtime::InteropServices::CriticalHandle{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Runtime::InteropServices::CriticalHandle,CriticalHandleZeroOrMinusOneIsInvalid>(v)
    }} 
    pub type SafeHandleMinusOneIsInvalid =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","Microsoft.Win32.SafeHandles.SafeHandleMinusOneIsInvalid">;
    use super::super::super::*;
    impl From<SafeHandleMinusOneIsInvalid> for System::Runtime::InteropServices::SafeHandle {
     fn from(v:SafeHandleMinusOneIsInvalid)->System::Runtime::InteropServices::SafeHandle{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Runtime::InteropServices::SafeHandle,SafeHandleMinusOneIsInvalid>(v)
    }} 
    pub type SafeHandleZeroOrMinusOneIsInvalid =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","Microsoft.Win32.SafeHandles.SafeHandleZeroOrMinusOneIsInvalid">;
    use super::super::super::*;
    impl From<SafeHandleZeroOrMinusOneIsInvalid> for System::Runtime::InteropServices::SafeHandle {
     fn from(v:SafeHandleZeroOrMinusOneIsInvalid)->System::Runtime::InteropServices::SafeHandle{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Runtime::InteropServices::SafeHandle,SafeHandleZeroOrMinusOneIsInvalid>(v)
    }} 
    pub type SafeFileHandle =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","Microsoft.Win32.SafeHandles.SafeFileHandle">;
    use super::super::super::*;
    impl From<SafeFileHandle> for Microsoft::Win32::SafeHandles::SafeHandleZeroOrMinusOneIsInvalid {
     fn from(v:SafeFileHandle)->Microsoft::Win32::SafeHandles::SafeHandleZeroOrMinusOneIsInvalid{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<Microsoft::Win32::SafeHandles::SafeHandleZeroOrMinusOneIsInvalid,SafeFileHandle>(v)
    }} 
    pub type SafeWaitHandle =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","Microsoft.Win32.SafeHandles.SafeWaitHandle">;
    use super::super::super::*;
    impl From<SafeWaitHandle> for Microsoft::Win32::SafeHandles::SafeHandleZeroOrMinusOneIsInvalid {
     fn from(v:SafeWaitHandle)->Microsoft::Win32::SafeHandles::SafeHandleZeroOrMinusOneIsInvalid{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<Microsoft::Win32::SafeHandles::SafeHandleZeroOrMinusOneIsInvalid,SafeWaitHandle>(v)
    }} 
    }
    }
    }
    pub mod System{
    pub mod Resources{
    pub type ManifestBasedResourceGroveler =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Resources.ManifestBasedResourceGroveler">;
    use super::super::*;
    impl From<ManifestBasedResourceGroveler> for System::Object {
     fn from(v:ManifestBasedResourceGroveler)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,ManifestBasedResourceGroveler>(v)
    }} 
    pub type FastResourceComparer =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Resources.FastResourceComparer">;
    use super::super::*;
    impl From<FastResourceComparer> for System::Object {
     fn from(v:FastResourceComparer)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,FastResourceComparer>(v)
    }} 
    pub type FileBasedResourceGroveler =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Resources.FileBasedResourceGroveler">;
    use super::super::*;
    impl From<FileBasedResourceGroveler> for System::Object {
     fn from(v:FileBasedResourceGroveler)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,FileBasedResourceGroveler>(v)
    }} 
    pub type IResourceGroveler =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Resources.IResourceGroveler">;
    use super::super::*;
    pub type IResourceReader =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Resources.IResourceReader">;
    use super::super::*;
    pub type MissingManifestResourceException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Resources.MissingManifestResourceException">;
    use super::super::*;
    impl From<MissingManifestResourceException> for System::SystemException {
     fn from(v:MissingManifestResourceException)->System::SystemException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::SystemException,MissingManifestResourceException>(v)
    }} 
    pub type MissingSatelliteAssemblyException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Resources.MissingSatelliteAssemblyException">;
    use super::super::*;
    impl From<MissingSatelliteAssemblyException> for System::SystemException {
     fn from(v:MissingSatelliteAssemblyException)->System::SystemException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::SystemException,MissingSatelliteAssemblyException>(v)
    }} 
    pub type NeutralResourcesLanguageAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Resources.NeutralResourcesLanguageAttribute">;
    use super::super::*;
    impl From<NeutralResourcesLanguageAttribute> for System::Attribute {
     fn from(v:NeutralResourcesLanguageAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,NeutralResourcesLanguageAttribute>(v)
    }} 
    pub type ResourceFallbackManager =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Resources.ResourceFallbackManager">;
    use super::super::*;
    impl From<ResourceFallbackManager> for System::Object {
     fn from(v:ResourceFallbackManager)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,ResourceFallbackManager>(v)
    }} 
    pub type ResourceManager =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Resources.ResourceManager">;
    use super::super::*;
    impl From<ResourceManager> for System::Object {
     fn from(v:ResourceManager)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,ResourceManager>(v)
    }} 
    pub type ResourceReader =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Resources.ResourceReader">;
    use super::super::*;
    impl From<ResourceReader> for System::Object {
     fn from(v:ResourceReader)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,ResourceReader>(v)
    }} 
    pub type ResourceSet =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Resources.ResourceSet">;
    use super::super::*;
    impl From<ResourceSet> for System::Object {
     fn from(v:ResourceSet)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,ResourceSet>(v)
    }} 
    pub type RuntimeResourceSet =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Resources.RuntimeResourceSet">;
    use super::super::*;
    impl From<RuntimeResourceSet> for System::Resources::ResourceSet {
     fn from(v:RuntimeResourceSet)->System::Resources::ResourceSet{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Resources::ResourceSet,RuntimeResourceSet>(v)
    }} 
    pub type SatelliteContractVersionAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Resources.SatelliteContractVersionAttribute">;
    use super::super::*;
    impl From<SatelliteContractVersionAttribute> for System::Attribute {
     fn from(v:SatelliteContractVersionAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,SatelliteContractVersionAttribute>(v)
    }} 
    }
    pub mod IO{
    pub mod Strategies{
    pub type BufferedFileStreamStrategy =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.IO.Strategies.BufferedFileStreamStrategy">;
    use super::super::super::*;
    impl From<BufferedFileStreamStrategy> for System::IO::Strategies::FileStreamStrategy {
     fn from(v:BufferedFileStreamStrategy)->System::IO::Strategies::FileStreamStrategy{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::IO::Strategies::FileStreamStrategy,BufferedFileStreamStrategy>(v)
    }} 
    pub type DerivedFileStreamStrategy =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.IO.Strategies.DerivedFileStreamStrategy">;
    use super::super::super::*;
    impl From<DerivedFileStreamStrategy> for System::IO::Strategies::FileStreamStrategy {
     fn from(v:DerivedFileStreamStrategy)->System::IO::Strategies::FileStreamStrategy{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::IO::Strategies::FileStreamStrategy,DerivedFileStreamStrategy>(v)
    }} 
    pub type FileStreamHelpers =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.IO.Strategies.FileStreamHelpers">;
    use super::super::super::*;
    impl From<FileStreamHelpers> for System::Object {
     fn from(v:FileStreamHelpers)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,FileStreamHelpers>(v)
    }} 
    pub type FileStreamStrategy =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.IO.Strategies.FileStreamStrategy">;
    use super::super::super::*;
    impl From<FileStreamStrategy> for System::IO::Stream {
     fn from(v:FileStreamStrategy)->System::IO::Stream{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::IO::Stream,FileStreamStrategy>(v)
    }} 
    pub type OSFileStreamStrategy =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.IO.Strategies.OSFileStreamStrategy">;
    use super::super::super::*;
    impl From<OSFileStreamStrategy> for System::IO::Strategies::FileStreamStrategy {
     fn from(v:OSFileStreamStrategy)->System::IO::Strategies::FileStreamStrategy{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::IO::Strategies::FileStreamStrategy,OSFileStreamStrategy>(v)
    }} 
    pub type UnixFileStreamStrategy =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.IO.Strategies.UnixFileStreamStrategy">;
    use super::super::super::*;
    impl From<UnixFileStreamStrategy> for System::IO::Strategies::OSFileStreamStrategy {
     fn from(v:UnixFileStreamStrategy)->System::IO::Strategies::OSFileStreamStrategy{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::IO::Strategies::OSFileStreamStrategy,UnixFileStreamStrategy>(v)
    }} 
    }
    pub mod Enumeration{
    pub type FileSystemEnumerableFactory =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.IO.Enumeration.FileSystemEnumerableFactory">;
    use super::super::super::*;
    impl From<FileSystemEnumerableFactory> for System::Object {
     fn from(v:FileSystemEnumerableFactory)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,FileSystemEnumerableFactory>(v)
    }} 
    pub type FileSystemName =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.IO.Enumeration.FileSystemName">;
    use super::super::super::*;
    impl From<FileSystemName> for System::Object {
     fn from(v:FileSystemName)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,FileSystemName>(v)
    }} 
    }
    pub type FileLoadException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.IO.FileLoadException">;
    use super::super::*;
    impl From<FileLoadException> for System::IO::IOException {
     fn from(v:FileLoadException)->System::IO::IOException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::IO::IOException,FileLoadException>(v)
    }} 
    pub type FileNotFoundException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.IO.FileNotFoundException">;
    use super::super::*;
    impl From<FileNotFoundException> for System::IO::IOException {
     fn from(v:FileNotFoundException)->System::IO::IOException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::IO::IOException,FileNotFoundException>(v)
    }} 
    pub type BinaryReader =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.IO.BinaryReader">;
    use super::super::*;
    impl From<BinaryReader> for System::Object {
     fn from(v:BinaryReader)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,BinaryReader>(v)
    }} 
    pub type BinaryWriter =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.IO.BinaryWriter">;
    use super::super::*;
    impl From<BinaryWriter> for System::Object {
     fn from(v:BinaryWriter)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,BinaryWriter>(v)
    }} 
    pub type BufferedStream =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.IO.BufferedStream">;
    use super::super::*;
    impl From<BufferedStream> for System::IO::Stream {
     fn from(v:BufferedStream)->System::IO::Stream{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::IO::Stream,BufferedStream>(v)
    }} 
    pub type Directory =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.IO.Directory">;
    use super::super::*;
    impl From<Directory> for System::Object {
     fn from(v:Directory)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Directory>(v)
    }} 
    pub type DirectoryInfo =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.IO.DirectoryInfo">;
    use super::super::*;
    impl From<DirectoryInfo> for System::IO::FileSystemInfo {
     fn from(v:DirectoryInfo)->System::IO::FileSystemInfo{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::IO::FileSystemInfo,DirectoryInfo>(v)
    }} 
    pub type DirectoryNotFoundException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.IO.DirectoryNotFoundException">;
    use super::super::*;
    impl From<DirectoryNotFoundException> for System::IO::IOException {
     fn from(v:DirectoryNotFoundException)->System::IO::IOException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::IO::IOException,DirectoryNotFoundException>(v)
    }} 
    pub type EncodingCache =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.IO.EncodingCache">;
    use super::super::*;
    impl From<EncodingCache> for System::Object {
     fn from(v:EncodingCache)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,EncodingCache>(v)
    }} 
    pub type EnumerationOptions =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.IO.EnumerationOptions">;
    use super::super::*;
    impl From<EnumerationOptions> for System::Object {
     fn from(v:EnumerationOptions)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,EnumerationOptions>(v)
    }} 
    pub type EndOfStreamException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.IO.EndOfStreamException">;
    use super::super::*;
    impl From<EndOfStreamException> for System::IO::IOException {
     fn from(v:EndOfStreamException)->System::IO::IOException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::IO::IOException,EndOfStreamException>(v)
    }} 
    pub type File =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.IO.File">;
    use super::super::*;
    impl From<File> for System::Object {
     fn from(v:File)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,File>(v)
    }} 
    pub type FileInfo =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.IO.FileInfo">;
    use super::super::*;
    impl From<FileInfo> for System::IO::FileSystemInfo {
     fn from(v:FileInfo)->System::IO::FileSystemInfo{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::IO::FileSystemInfo,FileInfo>(v)
    }} 
    pub type FileStream =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.IO.FileStream">;
    use super::super::*;
    impl From<FileStream> for System::IO::Stream {
     fn from(v:FileStream)->System::IO::Stream{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::IO::Stream,FileStream>(v)
    }} 
    pub type FileStreamOptions =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.IO.FileStreamOptions">;
    use super::super::*;
    impl From<FileStreamOptions> for System::Object {
     fn from(v:FileStreamOptions)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,FileStreamOptions>(v)
    }} 
    pub type FileSystem =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.IO.FileSystem">;
    use super::super::*;
    impl From<FileSystem> for System::Object {
     fn from(v:FileSystem)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,FileSystem>(v)
    }} 
    pub type FileSystemInfo =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.IO.FileSystemInfo">;
    use super::super::*;
    impl From<FileSystemInfo> for System::MarshalByRefObject {
     fn from(v:FileSystemInfo)->System::MarshalByRefObject{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::MarshalByRefObject,FileSystemInfo>(v)
    }} 
    pub type InvalidDataException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.IO.InvalidDataException">;
    use super::super::*;
    impl From<InvalidDataException> for System::SystemException {
     fn from(v:InvalidDataException)->System::SystemException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::SystemException,InvalidDataException>(v)
    }} 
    pub type IOException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.IO.IOException">;
    use super::super::*;
    impl From<IOException> for System::SystemException {
     fn from(v:IOException)->System::SystemException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::SystemException,IOException>(v)
    }} 
    pub type MemoryStream =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.IO.MemoryStream">;
    use super::super::*;
    impl From<MemoryStream> for System::IO::Stream {
     fn from(v:MemoryStream)->System::IO::Stream{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::IO::Stream,MemoryStream>(v)
    }} 
    pub type Path =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.IO.Path">;
    use super::super::*;
    impl From<Path> for System::Object {
     fn from(v:Path)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Path>(v)
    }} 
    pub type PathTooLongException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.IO.PathTooLongException">;
    use super::super::*;
    impl From<PathTooLongException> for System::IO::IOException {
     fn from(v:PathTooLongException)->System::IO::IOException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::IO::IOException,PathTooLongException>(v)
    }} 
    pub type PinnedBufferMemoryStream =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.IO.PinnedBufferMemoryStream">;
    use super::super::*;
    impl From<PinnedBufferMemoryStream> for System::IO::UnmanagedMemoryStream {
     fn from(v:PinnedBufferMemoryStream)->System::IO::UnmanagedMemoryStream{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::IO::UnmanagedMemoryStream,PinnedBufferMemoryStream>(v)
    }} 
    pub type RandomAccess =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.IO.RandomAccess">;
    use super::super::*;
    impl From<RandomAccess> for System::Object {
     fn from(v:RandomAccess)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,RandomAccess>(v)
    }} 
    pub type ReadLinesIterator =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.IO.ReadLinesIterator">;
    use super::super::*;
    pub type Stream =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.IO.Stream">;
    use super::super::*;
    impl From<Stream> for System::MarshalByRefObject {
     fn from(v:Stream)->System::MarshalByRefObject{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::MarshalByRefObject,Stream>(v)
    }} 
    pub type StreamReader =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.IO.StreamReader">;
    use super::super::*;
    impl From<StreamReader> for System::IO::TextReader {
     fn from(v:StreamReader)->System::IO::TextReader{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::IO::TextReader,StreamReader>(v)
    }} 
    pub type StreamWriter =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.IO.StreamWriter">;
    use super::super::*;
    impl From<StreamWriter> for System::IO::TextWriter {
     fn from(v:StreamWriter)->System::IO::TextWriter{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::IO::TextWriter,StreamWriter>(v)
    }} 
    pub type StringReader =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.IO.StringReader">;
    use super::super::*;
    impl From<StringReader> for System::IO::TextReader {
     fn from(v:StringReader)->System::IO::TextReader{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::IO::TextReader,StringReader>(v)
    }} 
    pub type StringWriter =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.IO.StringWriter">;
    use super::super::*;
    impl From<StringWriter> for System::IO::TextWriter {
     fn from(v:StringWriter)->System::IO::TextWriter{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::IO::TextWriter,StringWriter>(v)
    }} 
    pub type TextReader =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.IO.TextReader">;
    use super::super::*;
    impl From<TextReader> for System::MarshalByRefObject {
     fn from(v:TextReader)->System::MarshalByRefObject{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::MarshalByRefObject,TextReader>(v)
    }} 
    pub type TextWriter =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.IO.TextWriter">;
    use super::super::*;
    impl From<TextWriter> for System::MarshalByRefObject {
     fn from(v:TextWriter)->System::MarshalByRefObject{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::MarshalByRefObject,TextWriter>(v)
    }} 
    pub type UnmanagedMemoryAccessor =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.IO.UnmanagedMemoryAccessor">;
    use super::super::*;
    impl From<UnmanagedMemoryAccessor> for System::Object {
     fn from(v:UnmanagedMemoryAccessor)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,UnmanagedMemoryAccessor>(v)
    }} 
    pub type UnmanagedMemoryStream =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.IO.UnmanagedMemoryStream">;
    use super::super::*;
    impl From<UnmanagedMemoryStream> for System::IO::Stream {
     fn from(v:UnmanagedMemoryStream)->System::IO::Stream{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::IO::Stream,UnmanagedMemoryStream>(v)
    }} 
    pub type UnmanagedMemoryStreamWrapper =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.IO.UnmanagedMemoryStreamWrapper">;
    use super::super::*;
    impl From<UnmanagedMemoryStreamWrapper> for System::IO::MemoryStream {
     fn from(v:UnmanagedMemoryStreamWrapper)->System::IO::MemoryStream{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::IO::MemoryStream,UnmanagedMemoryStreamWrapper>(v)
    }} 
    pub type PathInternal =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.IO.PathInternal">;
    use super::super::*;
    impl From<PathInternal> for System::Object {
     fn from(v:PathInternal)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,PathInternal>(v)
    }} 
    pub type Win32Marshal =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.IO.Win32Marshal">;
    use super::super::*;
    impl From<Win32Marshal> for System::Object {
     fn from(v:Win32Marshal)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Win32Marshal>(v)
    }} 
    pub type PersistedFiles =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.IO.PersistedFiles">;
    use super::super::*;
    impl From<PersistedFiles> for System::Object {
     fn from(v:PersistedFiles)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,PersistedFiles>(v)
    }} 
    pub type DriveInfoInternal =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.IO.DriveInfoInternal">;
    use super::super::*;
    impl From<DriveInfoInternal> for System::Object {
     fn from(v:DriveInfoInternal)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,DriveInfoInternal>(v)
    }} 
    }
    pub mod Security{
    pub mod Principal{
    pub type IIdentity =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Security.Principal.IIdentity">;
    use super::super::super::*;
    pub type IPrincipal =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Security.Principal.IPrincipal">;
    use super::super::super::*;
    }
    pub mod Cryptography{
    pub type CryptographicException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Security.Cryptography.CryptographicException">;
    use super::super::super::*;
    impl From<CryptographicException> for System::SystemException {
     fn from(v:CryptographicException)->System::SystemException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::SystemException,CryptographicException>(v)
    }} 
    }
    pub mod Permissions{
    pub type CodeAccessSecurityAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Security.Permissions.CodeAccessSecurityAttribute">;
    use super::super::super::*;
    impl From<CodeAccessSecurityAttribute> for System::Security::Permissions::SecurityAttribute {
     fn from(v:CodeAccessSecurityAttribute)->System::Security::Permissions::SecurityAttribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Security::Permissions::SecurityAttribute,CodeAccessSecurityAttribute>(v)
    }} 
    pub type SecurityAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Security.Permissions.SecurityAttribute">;
    use super::super::super::*;
    impl From<SecurityAttribute> for System::Attribute {
     fn from(v:SecurityAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,SecurityAttribute>(v)
    }} 
    pub type SecurityPermissionAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Security.Permissions.SecurityPermissionAttribute">;
    use super::super::super::*;
    impl From<SecurityPermissionAttribute> for System::Security::Permissions::CodeAccessSecurityAttribute {
     fn from(v:SecurityPermissionAttribute)->System::Security::Permissions::CodeAccessSecurityAttribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Security::Permissions::CodeAccessSecurityAttribute,SecurityPermissionAttribute>(v)
    }} 
    }
    pub type AllowPartiallyTrustedCallersAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Security.AllowPartiallyTrustedCallersAttribute">;
    use super::super::*;
    impl From<AllowPartiallyTrustedCallersAttribute> for System::Attribute {
     fn from(v:AllowPartiallyTrustedCallersAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,AllowPartiallyTrustedCallersAttribute>(v)
    }} 
    pub type IPermission =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Security.IPermission">;
    use super::super::*;
    pub type ISecurityEncodable =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Security.ISecurityEncodable">;
    use super::super::*;
    pub type IStackWalk =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Security.IStackWalk">;
    use super::super::*;
    pub type PermissionSet =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Security.PermissionSet">;
    use super::super::*;
    impl From<PermissionSet> for System::Object {
     fn from(v:PermissionSet)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,PermissionSet>(v)
    }} 
    pub type SecureString =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Security.SecureString">;
    use super::super::*;
    impl From<SecureString> for System::Object {
     fn from(v:SecureString)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,SecureString>(v)
    }} 
    pub type SecurityCriticalAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Security.SecurityCriticalAttribute">;
    use super::super::*;
    impl From<SecurityCriticalAttribute> for System::Attribute {
     fn from(v:SecurityCriticalAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,SecurityCriticalAttribute>(v)
    }} 
    pub type SecurityElement =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Security.SecurityElement">;
    use super::super::*;
    impl From<SecurityElement> for System::Object {
     fn from(v:SecurityElement)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,SecurityElement>(v)
    }} 
    pub type SecurityException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Security.SecurityException">;
    use super::super::*;
    impl From<SecurityException> for System::SystemException {
     fn from(v:SecurityException)->System::SystemException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::SystemException,SecurityException>(v)
    }} 
    pub type SecurityRulesAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Security.SecurityRulesAttribute">;
    use super::super::*;
    impl From<SecurityRulesAttribute> for System::Attribute {
     fn from(v:SecurityRulesAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,SecurityRulesAttribute>(v)
    }} 
    pub type SecuritySafeCriticalAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Security.SecuritySafeCriticalAttribute">;
    use super::super::*;
    impl From<SecuritySafeCriticalAttribute> for System::Attribute {
     fn from(v:SecuritySafeCriticalAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,SecuritySafeCriticalAttribute>(v)
    }} 
    pub type SecurityTransparentAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Security.SecurityTransparentAttribute">;
    use super::super::*;
    impl From<SecurityTransparentAttribute> for System::Attribute {
     fn from(v:SecurityTransparentAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,SecurityTransparentAttribute>(v)
    }} 
    pub type SecurityTreatAsSafeAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Security.SecurityTreatAsSafeAttribute">;
    use super::super::*;
    impl From<SecurityTreatAsSafeAttribute> for System::Attribute {
     fn from(v:SecurityTreatAsSafeAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,SecurityTreatAsSafeAttribute>(v)
    }} 
    pub type SuppressUnmanagedCodeSecurityAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Security.SuppressUnmanagedCodeSecurityAttribute">;
    use super::super::*;
    impl From<SuppressUnmanagedCodeSecurityAttribute> for System::Attribute {
     fn from(v:SuppressUnmanagedCodeSecurityAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,SuppressUnmanagedCodeSecurityAttribute>(v)
    }} 
    pub type UnverifiableCodeAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Security.UnverifiableCodeAttribute">;
    use super::super::*;
    impl From<UnverifiableCodeAttribute> for System::Attribute {
     fn from(v:UnverifiableCodeAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,UnverifiableCodeAttribute>(v)
    }} 
    pub type VerificationException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Security.VerificationException">;
    use super::super::*;
    impl From<VerificationException> for System::SystemException {
     fn from(v:VerificationException)->System::SystemException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::SystemException,VerificationException>(v)
    }} 
    }
    pub mod Numerics{
    pub type BitOperations =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Numerics.BitOperations">;
    use super::super::*;
    impl From<BitOperations> for System::Object {
     fn from(v:BitOperations)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,BitOperations>(v)
    }} 
    pub type Vector =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Numerics.Vector">;
    use super::super::*;
    impl From<Vector> for System::Object {
     fn from(v:Vector)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Vector>(v)
    }} 
    pub type VectorMath =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Numerics.VectorMath">;
    use super::super::*;
    impl From<VectorMath> for System::Object {
     fn from(v:VectorMath)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,VectorMath>(v)
    }} 
    pub type Crc32ReflectedTable =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Numerics.Crc32ReflectedTable">;
    use super::super::*;
    impl From<Crc32ReflectedTable> for System::Object {
     fn from(v:Crc32ReflectedTable)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Crc32ReflectedTable>(v)
    }} 
    }
    pub mod ComponentModel{
    pub type DefaultValueAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.ComponentModel.DefaultValueAttribute">;
    use super::super::*;
    impl From<DefaultValueAttribute> for System::Attribute {
     fn from(v:DefaultValueAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,DefaultValueAttribute>(v)
    }} 
    pub type EditorBrowsableAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.ComponentModel.EditorBrowsableAttribute">;
    use super::super::*;
    impl From<EditorBrowsableAttribute> for System::Attribute {
     fn from(v:EditorBrowsableAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,EditorBrowsableAttribute>(v)
    }} 
    pub type Win32Exception =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.ComponentModel.Win32Exception">;
    use super::super::*;
    impl From<Win32Exception> for System::Runtime::InteropServices::ExternalException {
     fn from(v:Win32Exception)->System::Runtime::InteropServices::ExternalException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Runtime::InteropServices::ExternalException,Win32Exception>(v)
    }} 
    }
    pub mod StubHelpers{
    pub type AnsiCharMarshaler =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.StubHelpers.AnsiCharMarshaler">;
    use super::super::*;
    impl From<AnsiCharMarshaler> for System::Object {
     fn from(v:AnsiCharMarshaler)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,AnsiCharMarshaler>(v)
    }} 
    pub type CSTRMarshaler =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.StubHelpers.CSTRMarshaler">;
    use super::super::*;
    impl From<CSTRMarshaler> for System::Object {
     fn from(v:CSTRMarshaler)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,CSTRMarshaler>(v)
    }} 
    pub type UTF8BufferMarshaler =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.StubHelpers.UTF8BufferMarshaler">;
    use super::super::*;
    impl From<UTF8BufferMarshaler> for System::Object {
     fn from(v:UTF8BufferMarshaler)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,UTF8BufferMarshaler>(v)
    }} 
    pub type BSTRMarshaler =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.StubHelpers.BSTRMarshaler">;
    use super::super::*;
    impl From<BSTRMarshaler> for System::Object {
     fn from(v:BSTRMarshaler)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,BSTRMarshaler>(v)
    }} 
    pub type VBByValStrMarshaler =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.StubHelpers.VBByValStrMarshaler">;
    use super::super::*;
    impl From<VBByValStrMarshaler> for System::Object {
     fn from(v:VBByValStrMarshaler)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,VBByValStrMarshaler>(v)
    }} 
    pub type AnsiBSTRMarshaler =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.StubHelpers.AnsiBSTRMarshaler">;
    use super::super::*;
    impl From<AnsiBSTRMarshaler> for System::Object {
     fn from(v:AnsiBSTRMarshaler)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,AnsiBSTRMarshaler>(v)
    }} 
    pub type FixedWSTRMarshaler =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.StubHelpers.FixedWSTRMarshaler">;
    use super::super::*;
    impl From<FixedWSTRMarshaler> for System::Object {
     fn from(v:FixedWSTRMarshaler)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,FixedWSTRMarshaler>(v)
    }} 
    pub type HandleMarshaler =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.StubHelpers.HandleMarshaler">;
    use super::super::*;
    impl From<HandleMarshaler> for System::Object {
     fn from(v:HandleMarshaler)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,HandleMarshaler>(v)
    }} 
    pub type DateMarshaler =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.StubHelpers.DateMarshaler">;
    use super::super::*;
    impl From<DateMarshaler> for System::Object {
     fn from(v:DateMarshaler)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,DateMarshaler>(v)
    }} 
    pub type MngdNativeArrayMarshaler =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.StubHelpers.MngdNativeArrayMarshaler">;
    use super::super::*;
    impl From<MngdNativeArrayMarshaler> for System::Object {
     fn from(v:MngdNativeArrayMarshaler)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,MngdNativeArrayMarshaler>(v)
    }} 
    pub type MngdFixedArrayMarshaler =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.StubHelpers.MngdFixedArrayMarshaler">;
    use super::super::*;
    impl From<MngdFixedArrayMarshaler> for System::Object {
     fn from(v:MngdFixedArrayMarshaler)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,MngdFixedArrayMarshaler>(v)
    }} 
    pub type MngdRefCustomMarshaler =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.StubHelpers.MngdRefCustomMarshaler">;
    use super::super::*;
    impl From<MngdRefCustomMarshaler> for System::Object {
     fn from(v:MngdRefCustomMarshaler)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,MngdRefCustomMarshaler>(v)
    }} 
    pub type CleanupWorkListElement =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.StubHelpers.CleanupWorkListElement">;
    use super::super::*;
    impl From<CleanupWorkListElement> for System::Object {
     fn from(v:CleanupWorkListElement)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,CleanupWorkListElement>(v)
    }} 
    pub type KeepAliveCleanupWorkListElement =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.StubHelpers.KeepAliveCleanupWorkListElement">;
    use super::super::*;
    impl From<KeepAliveCleanupWorkListElement> for System::StubHelpers::CleanupWorkListElement {
     fn from(v:KeepAliveCleanupWorkListElement)->System::StubHelpers::CleanupWorkListElement{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::StubHelpers::CleanupWorkListElement,KeepAliveCleanupWorkListElement>(v)
    }} 
    pub type SafeHandleCleanupWorkListElement =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.StubHelpers.SafeHandleCleanupWorkListElement">;
    use super::super::*;
    impl From<SafeHandleCleanupWorkListElement> for System::StubHelpers::CleanupWorkListElement {
     fn from(v:SafeHandleCleanupWorkListElement)->System::StubHelpers::CleanupWorkListElement{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::StubHelpers::CleanupWorkListElement,SafeHandleCleanupWorkListElement>(v)
    }} 
    pub type StubHelpers =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.StubHelpers.StubHelpers">;
    use super::super::*;
    impl From<StubHelpers> for System::Object {
     fn from(v:StubHelpers)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,StubHelpers>(v)
    }} 
    }
    pub mod Diagnostics{
    pub mod Tracing{
    pub type EventPipeInternal =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Tracing.EventPipeInternal">;
    use super::super::super::*;
    impl From<EventPipeInternal> for System::Object {
     fn from(v:EventPipeInternal)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,EventPipeInternal>(v)
    }} 
    pub type NativeRuntimeEventSource =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Tracing.NativeRuntimeEventSource">;
    use super::super::super::*;
    impl From<NativeRuntimeEventSource> for System::Diagnostics::Tracing::EventSource {
     fn from(v:NativeRuntimeEventSource)->System::Diagnostics::Tracing::EventSource{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Diagnostics::Tracing::EventSource,NativeRuntimeEventSource>(v)
    }} 
    pub type ActivityTracker =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Tracing.ActivityTracker">;
    use super::super::super::*;
    impl From<ActivityTracker> for System::Object {
     fn from(v:ActivityTracker)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,ActivityTracker>(v)
    }} 
    pub type DiagnosticCounter =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Tracing.DiagnosticCounter">;
    use super::super::super::*;
    impl From<DiagnosticCounter> for System::Object {
     fn from(v:DiagnosticCounter)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,DiagnosticCounter>(v)
    }} 
    pub type CounterGroup =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Tracing.CounterGroup">;
    use super::super::super::*;
    impl From<CounterGroup> for System::Object {
     fn from(v:CounterGroup)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,CounterGroup>(v)
    }} 
    pub type CounterPayload =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Tracing.CounterPayload">;
    use super::super::super::*;
    impl From<CounterPayload> for System::Object {
     fn from(v:CounterPayload)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,CounterPayload>(v)
    }} 
    pub type IncrementingCounterPayload =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Tracing.IncrementingCounterPayload">;
    use super::super::super::*;
    impl From<IncrementingCounterPayload> for System::Object {
     fn from(v:IncrementingCounterPayload)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,IncrementingCounterPayload>(v)
    }} 
    pub type EventCounter =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Tracing.EventCounter">;
    use super::super::super::*;
    impl From<EventCounter> for System::Diagnostics::Tracing::DiagnosticCounter {
     fn from(v:EventCounter)->System::Diagnostics::Tracing::DiagnosticCounter{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Diagnostics::Tracing::DiagnosticCounter,EventCounter>(v)
    }} 
    pub type CounterPayloadType =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Tracing.CounterPayloadType">;
    use super::super::super::*;
    impl From<CounterPayloadType> for System::Object {
     fn from(v:CounterPayloadType)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,CounterPayloadType>(v)
    }} 
    pub type EventPipeEventDispatcher =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Tracing.EventPipeEventDispatcher">;
    use super::super::super::*;
    impl From<EventPipeEventDispatcher> for System::Object {
     fn from(v:EventPipeEventDispatcher)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,EventPipeEventDispatcher>(v)
    }} 
    pub type EventPipeEventProvider =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Tracing.EventPipeEventProvider">;
    use super::super::super::*;
    impl From<EventPipeEventProvider> for System::Diagnostics::Tracing::EventProviderImpl {
     fn from(v:EventPipeEventProvider)->System::Diagnostics::Tracing::EventProviderImpl{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Diagnostics::Tracing::EventProviderImpl,EventPipeEventProvider>(v)
    }} 
    pub type EventPipeMetadataGenerator =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Tracing.EventPipeMetadataGenerator">;
    use super::super::super::*;
    impl From<EventPipeMetadataGenerator> for System::Object {
     fn from(v:EventPipeMetadataGenerator)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,EventPipeMetadataGenerator>(v)
    }} 
    pub type EventPipePayloadDecoder =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Tracing.EventPipePayloadDecoder">;
    use super::super::super::*;
    impl From<EventPipePayloadDecoder> for System::Object {
     fn from(v:EventPipePayloadDecoder)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,EventPipePayloadDecoder>(v)
    }} 
    pub type EventProvider =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Tracing.EventProvider">;
    use super::super::super::*;
    impl From<EventProvider> for System::Object {
     fn from(v:EventProvider)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,EventProvider>(v)
    }} 
    pub type EventProviderImpl =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Tracing.EventProviderImpl">;
    use super::super::super::*;
    impl From<EventProviderImpl> for System::Object {
     fn from(v:EventProviderImpl)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,EventProviderImpl>(v)
    }} 
    pub type EventSource =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Tracing.EventSource">;
    use super::super::super::*;
    impl From<EventSource> for System::Object {
     fn from(v:EventSource)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,EventSource>(v)
    }} 
    pub type EventListener =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Tracing.EventListener">;
    use super::super::super::*;
    impl From<EventListener> for System::Object {
     fn from(v:EventListener)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,EventListener>(v)
    }} 
    pub type EventCommandEventArgs =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Tracing.EventCommandEventArgs">;
    use super::super::super::*;
    impl From<EventCommandEventArgs> for System::EventArgs {
     fn from(v:EventCommandEventArgs)->System::EventArgs{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::EventArgs,EventCommandEventArgs>(v)
    }} 
    pub type EventSourceCreatedEventArgs =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Tracing.EventSourceCreatedEventArgs">;
    use super::super::super::*;
    impl From<EventSourceCreatedEventArgs> for System::EventArgs {
     fn from(v:EventSourceCreatedEventArgs)->System::EventArgs{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::EventArgs,EventSourceCreatedEventArgs>(v)
    }} 
    pub type EventWrittenEventArgs =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Tracing.EventWrittenEventArgs">;
    use super::super::super::*;
    impl From<EventWrittenEventArgs> for System::EventArgs {
     fn from(v:EventWrittenEventArgs)->System::EventArgs{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::EventArgs,EventWrittenEventArgs>(v)
    }} 
    pub type EventSourceAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Tracing.EventSourceAttribute">;
    use super::super::super::*;
    impl From<EventSourceAttribute> for System::Attribute {
     fn from(v:EventSourceAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,EventSourceAttribute>(v)
    }} 
    pub type EventAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Tracing.EventAttribute">;
    use super::super::super::*;
    impl From<EventAttribute> for System::Attribute {
     fn from(v:EventAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,EventAttribute>(v)
    }} 
    pub type NonEventAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Tracing.NonEventAttribute">;
    use super::super::super::*;
    impl From<NonEventAttribute> for System::Attribute {
     fn from(v:NonEventAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,NonEventAttribute>(v)
    }} 
    pub type EventChannelAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Tracing.EventChannelAttribute">;
    use super::super::super::*;
    impl From<EventChannelAttribute> for System::Attribute {
     fn from(v:EventChannelAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,EventChannelAttribute>(v)
    }} 
    pub type EventDispatcher =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Tracing.EventDispatcher">;
    use super::super::super::*;
    impl From<EventDispatcher> for System::Object {
     fn from(v:EventDispatcher)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,EventDispatcher>(v)
    }} 
    pub type ManifestBuilder =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Tracing.ManifestBuilder">;
    use super::super::super::*;
    impl From<ManifestBuilder> for System::Object {
     fn from(v:ManifestBuilder)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,ManifestBuilder>(v)
    }} 
    pub type EventSourceException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Tracing.EventSourceException">;
    use super::super::super::*;
    impl From<EventSourceException> for System::Exception {
     fn from(v:EventSourceException)->System::Exception{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Exception,EventSourceException>(v)
    }} 
    pub type FrameworkEventSource =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Tracing.FrameworkEventSource">;
    use super::super::super::*;
    impl From<FrameworkEventSource> for System::Diagnostics::Tracing::EventSource {
     fn from(v:FrameworkEventSource)->System::Diagnostics::Tracing::EventSource{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Diagnostics::Tracing::EventSource,FrameworkEventSource>(v)
    }} 
    pub type IncrementingEventCounter =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Tracing.IncrementingEventCounter">;
    use super::super::super::*;
    impl From<IncrementingEventCounter> for System::Diagnostics::Tracing::DiagnosticCounter {
     fn from(v:IncrementingEventCounter)->System::Diagnostics::Tracing::DiagnosticCounter{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Diagnostics::Tracing::DiagnosticCounter,IncrementingEventCounter>(v)
    }} 
    pub type IncrementingEventCounterPayloadType =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Tracing.IncrementingEventCounterPayloadType">;
    use super::super::super::*;
    impl From<IncrementingEventCounterPayloadType> for System::Object {
     fn from(v:IncrementingEventCounterPayloadType)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,IncrementingEventCounterPayloadType>(v)
    }} 
    pub type IncrementingPollingCounter =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Tracing.IncrementingPollingCounter">;
    use super::super::super::*;
    impl From<IncrementingPollingCounter> for System::Diagnostics::Tracing::DiagnosticCounter {
     fn from(v:IncrementingPollingCounter)->System::Diagnostics::Tracing::DiagnosticCounter{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Diagnostics::Tracing::DiagnosticCounter,IncrementingPollingCounter>(v)
    }} 
    pub type IncrementingPollingCounterPayloadType =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Tracing.IncrementingPollingCounterPayloadType">;
    use super::super::super::*;
    impl From<IncrementingPollingCounterPayloadType> for System::Object {
     fn from(v:IncrementingPollingCounterPayloadType)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,IncrementingPollingCounterPayloadType>(v)
    }} 
    pub type PollingCounter =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Tracing.PollingCounter">;
    use super::super::super::*;
    impl From<PollingCounter> for System::Diagnostics::Tracing::DiagnosticCounter {
     fn from(v:PollingCounter)->System::Diagnostics::Tracing::DiagnosticCounter{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Diagnostics::Tracing::DiagnosticCounter,PollingCounter>(v)
    }} 
    pub type PollingPayloadType =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Tracing.PollingPayloadType">;
    use super::super::super::*;
    impl From<PollingPayloadType> for System::Object {
     fn from(v:PollingPayloadType)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,PollingPayloadType>(v)
    }} 
    pub type RuntimeEventSource =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Tracing.RuntimeEventSource">;
    use super::super::super::*;
    impl From<RuntimeEventSource> for System::Diagnostics::Tracing::EventSource {
     fn from(v:RuntimeEventSource)->System::Diagnostics::Tracing::EventSource{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Diagnostics::Tracing::EventSource,RuntimeEventSource>(v)
    }} 
    pub type ArrayTypeInfo =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Tracing.ArrayTypeInfo">;
    use super::super::super::*;
    impl From<ArrayTypeInfo> for System::Diagnostics::Tracing::TraceLoggingTypeInfo {
     fn from(v:ArrayTypeInfo)->System::Diagnostics::Tracing::TraceLoggingTypeInfo{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Diagnostics::Tracing::TraceLoggingTypeInfo,ArrayTypeInfo>(v)
    }} 
    pub type EnumerableTypeInfo =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Tracing.EnumerableTypeInfo">;
    use super::super::super::*;
    impl From<EnumerableTypeInfo> for System::Diagnostics::Tracing::TraceLoggingTypeInfo {
     fn from(v:EnumerableTypeInfo)->System::Diagnostics::Tracing::TraceLoggingTypeInfo{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Diagnostics::Tracing::TraceLoggingTypeInfo,EnumerableTypeInfo>(v)
    }} 
    pub type EventDataAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Tracing.EventDataAttribute">;
    use super::super::super::*;
    impl From<EventDataAttribute> for System::Attribute {
     fn from(v:EventDataAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,EventDataAttribute>(v)
    }} 
    pub type EventFieldAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Tracing.EventFieldAttribute">;
    use super::super::super::*;
    impl From<EventFieldAttribute> for System::Attribute {
     fn from(v:EventFieldAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,EventFieldAttribute>(v)
    }} 
    pub type EventIgnoreAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Tracing.EventIgnoreAttribute">;
    use super::super::super::*;
    impl From<EventIgnoreAttribute> for System::Attribute {
     fn from(v:EventIgnoreAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,EventIgnoreAttribute>(v)
    }} 
    pub type EventPayload =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Tracing.EventPayload">;
    use super::super::super::*;
    impl From<EventPayload> for System::Object {
     fn from(v:EventPayload)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,EventPayload>(v)
    }} 
    pub type FieldMetadata =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Tracing.FieldMetadata">;
    use super::super::super::*;
    impl From<FieldMetadata> for System::Object {
     fn from(v:FieldMetadata)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,FieldMetadata>(v)
    }} 
    pub type InvokeTypeInfo =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Tracing.InvokeTypeInfo">;
    use super::super::super::*;
    impl From<InvokeTypeInfo> for System::Diagnostics::Tracing::TraceLoggingTypeInfo {
     fn from(v:InvokeTypeInfo)->System::Diagnostics::Tracing::TraceLoggingTypeInfo{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Diagnostics::Tracing::TraceLoggingTypeInfo,InvokeTypeInfo>(v)
    }} 
    pub type NameInfo =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Tracing.NameInfo">;
    use super::super::super::*;
    pub type PropertyAnalysis =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Tracing.PropertyAnalysis">;
    use super::super::super::*;
    impl From<PropertyAnalysis> for System::Object {
     fn from(v:PropertyAnalysis)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,PropertyAnalysis>(v)
    }} 
    pub type NullTypeInfo =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Tracing.NullTypeInfo">;
    use super::super::super::*;
    impl From<NullTypeInfo> for System::Diagnostics::Tracing::TraceLoggingTypeInfo {
     fn from(v:NullTypeInfo)->System::Diagnostics::Tracing::TraceLoggingTypeInfo{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Diagnostics::Tracing::TraceLoggingTypeInfo,NullTypeInfo>(v)
    }} 
    pub type ScalarTypeInfo =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Tracing.ScalarTypeInfo">;
    use super::super::super::*;
    impl From<ScalarTypeInfo> for System::Diagnostics::Tracing::TraceLoggingTypeInfo {
     fn from(v:ScalarTypeInfo)->System::Diagnostics::Tracing::TraceLoggingTypeInfo{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Diagnostics::Tracing::TraceLoggingTypeInfo,ScalarTypeInfo>(v)
    }} 
    pub type ScalarArrayTypeInfo =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Tracing.ScalarArrayTypeInfo">;
    use super::super::super::*;
    impl From<ScalarArrayTypeInfo> for System::Diagnostics::Tracing::TraceLoggingTypeInfo {
     fn from(v:ScalarArrayTypeInfo)->System::Diagnostics::Tracing::TraceLoggingTypeInfo{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Diagnostics::Tracing::TraceLoggingTypeInfo,ScalarArrayTypeInfo>(v)
    }} 
    pub type StringTypeInfo =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Tracing.StringTypeInfo">;
    use super::super::super::*;
    impl From<StringTypeInfo> for System::Diagnostics::Tracing::TraceLoggingTypeInfo {
     fn from(v:StringTypeInfo)->System::Diagnostics::Tracing::TraceLoggingTypeInfo{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Diagnostics::Tracing::TraceLoggingTypeInfo,StringTypeInfo>(v)
    }} 
    pub type DateTimeTypeInfo =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Tracing.DateTimeTypeInfo">;
    use super::super::super::*;
    impl From<DateTimeTypeInfo> for System::Diagnostics::Tracing::TraceLoggingTypeInfo {
     fn from(v:DateTimeTypeInfo)->System::Diagnostics::Tracing::TraceLoggingTypeInfo{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Diagnostics::Tracing::TraceLoggingTypeInfo,DateTimeTypeInfo>(v)
    }} 
    pub type DateTimeOffsetTypeInfo =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Tracing.DateTimeOffsetTypeInfo">;
    use super::super::super::*;
    impl From<DateTimeOffsetTypeInfo> for System::Diagnostics::Tracing::TraceLoggingTypeInfo {
     fn from(v:DateTimeOffsetTypeInfo)->System::Diagnostics::Tracing::TraceLoggingTypeInfo{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Diagnostics::Tracing::TraceLoggingTypeInfo,DateTimeOffsetTypeInfo>(v)
    }} 
    pub type TimeSpanTypeInfo =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Tracing.TimeSpanTypeInfo">;
    use super::super::super::*;
    impl From<TimeSpanTypeInfo> for System::Diagnostics::Tracing::TraceLoggingTypeInfo {
     fn from(v:TimeSpanTypeInfo)->System::Diagnostics::Tracing::TraceLoggingTypeInfo{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Diagnostics::Tracing::TraceLoggingTypeInfo,TimeSpanTypeInfo>(v)
    }} 
    pub type DecimalTypeInfo =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Tracing.DecimalTypeInfo">;
    use super::super::super::*;
    impl From<DecimalTypeInfo> for System::Diagnostics::Tracing::TraceLoggingTypeInfo {
     fn from(v:DecimalTypeInfo)->System::Diagnostics::Tracing::TraceLoggingTypeInfo{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Diagnostics::Tracing::TraceLoggingTypeInfo,DecimalTypeInfo>(v)
    }} 
    pub type NullableTypeInfo =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Tracing.NullableTypeInfo">;
    use super::super::super::*;
    impl From<NullableTypeInfo> for System::Diagnostics::Tracing::TraceLoggingTypeInfo {
     fn from(v:NullableTypeInfo)->System::Diagnostics::Tracing::TraceLoggingTypeInfo{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Diagnostics::Tracing::TraceLoggingTypeInfo,NullableTypeInfo>(v)
    }} 
    pub type Statics =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Tracing.Statics">;
    use super::super::super::*;
    impl From<Statics> for System::Object {
     fn from(v:Statics)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Statics>(v)
    }} 
    pub type TraceLoggingDataCollector =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Tracing.TraceLoggingDataCollector">;
    use super::super::super::*;
    impl From<TraceLoggingDataCollector> for System::Object {
     fn from(v:TraceLoggingDataCollector)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,TraceLoggingDataCollector>(v)
    }} 
    pub type TraceLoggingEventHandleTable =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Tracing.TraceLoggingEventHandleTable">;
    use super::super::super::*;
    impl From<TraceLoggingEventHandleTable> for System::Object {
     fn from(v:TraceLoggingEventHandleTable)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,TraceLoggingEventHandleTable>(v)
    }} 
    pub type TraceLoggingEventTypes =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Tracing.TraceLoggingEventTypes">;
    use super::super::super::*;
    impl From<TraceLoggingEventTypes> for System::Object {
     fn from(v:TraceLoggingEventTypes)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,TraceLoggingEventTypes>(v)
    }} 
    pub type TraceLoggingMetadataCollector =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Tracing.TraceLoggingMetadataCollector">;
    use super::super::super::*;
    impl From<TraceLoggingMetadataCollector> for System::Object {
     fn from(v:TraceLoggingMetadataCollector)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,TraceLoggingMetadataCollector>(v)
    }} 
    pub type TraceLoggingTypeInfo =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Tracing.TraceLoggingTypeInfo">;
    use super::super::super::*;
    impl From<TraceLoggingTypeInfo> for System::Object {
     fn from(v:TraceLoggingTypeInfo)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,TraceLoggingTypeInfo>(v)
    }} 
    pub type TypeAnalysis =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Tracing.TypeAnalysis">;
    use super::super::super::*;
    impl From<TypeAnalysis> for System::Object {
     fn from(v:TypeAnalysis)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,TypeAnalysis>(v)
    }} 
    pub type XplatEventLogger =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Tracing.XplatEventLogger">;
    use super::super::super::*;
    impl From<XplatEventLogger> for System::Diagnostics::Tracing::EventListener {
     fn from(v:XplatEventLogger)->System::Diagnostics::Tracing::EventListener{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Diagnostics::Tracing::EventListener,XplatEventLogger>(v)
    }} 
    pub type RuntimeEventSourceHelper =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Tracing.RuntimeEventSourceHelper">;
    use super::super::super::*;
    impl From<RuntimeEventSourceHelper> for System::Object {
     fn from(v:RuntimeEventSourceHelper)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,RuntimeEventSourceHelper>(v)
    }} 
    }
    pub mod SymbolStore{
    pub type ISymbolDocumentWriter =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.SymbolStore.ISymbolDocumentWriter">;
    use super::super::super::*;
    }
    pub mod CodeAnalysis{
    pub type ConstantExpectedAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.CodeAnalysis.ConstantExpectedAttribute">;
    use super::super::super::*;
    impl From<ConstantExpectedAttribute> for System::Attribute {
     fn from(v:ConstantExpectedAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,ConstantExpectedAttribute>(v)
    }} 
    pub type DynamicallyAccessedMembersAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.CodeAnalysis.DynamicallyAccessedMembersAttribute">;
    use super::super::super::*;
    impl From<DynamicallyAccessedMembersAttribute> for System::Attribute {
     fn from(v:DynamicallyAccessedMembersAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,DynamicallyAccessedMembersAttribute>(v)
    }} 
    pub type DynamicDependencyAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.CodeAnalysis.DynamicDependencyAttribute">;
    use super::super::super::*;
    impl From<DynamicDependencyAttribute> for System::Attribute {
     fn from(v:DynamicDependencyAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,DynamicDependencyAttribute>(v)
    }} 
    pub type ExcludeFromCodeCoverageAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.CodeAnalysis.ExcludeFromCodeCoverageAttribute">;
    use super::super::super::*;
    impl From<ExcludeFromCodeCoverageAttribute> for System::Attribute {
     fn from(v:ExcludeFromCodeCoverageAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,ExcludeFromCodeCoverageAttribute>(v)
    }} 
    pub type ExperimentalAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.CodeAnalysis.ExperimentalAttribute">;
    use super::super::super::*;
    impl From<ExperimentalAttribute> for System::Attribute {
     fn from(v:ExperimentalAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,ExperimentalAttribute>(v)
    }} 
    pub type AllowNullAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.CodeAnalysis.AllowNullAttribute">;
    use super::super::super::*;
    impl From<AllowNullAttribute> for System::Attribute {
     fn from(v:AllowNullAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,AllowNullAttribute>(v)
    }} 
    pub type DisallowNullAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.CodeAnalysis.DisallowNullAttribute">;
    use super::super::super::*;
    impl From<DisallowNullAttribute> for System::Attribute {
     fn from(v:DisallowNullAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,DisallowNullAttribute>(v)
    }} 
    pub type MaybeNullAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.CodeAnalysis.MaybeNullAttribute">;
    use super::super::super::*;
    impl From<MaybeNullAttribute> for System::Attribute {
     fn from(v:MaybeNullAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,MaybeNullAttribute>(v)
    }} 
    pub type NotNullAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.CodeAnalysis.NotNullAttribute">;
    use super::super::super::*;
    impl From<NotNullAttribute> for System::Attribute {
     fn from(v:NotNullAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,NotNullAttribute>(v)
    }} 
    pub type MaybeNullWhenAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.CodeAnalysis.MaybeNullWhenAttribute">;
    use super::super::super::*;
    impl From<MaybeNullWhenAttribute> for System::Attribute {
     fn from(v:MaybeNullWhenAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,MaybeNullWhenAttribute>(v)
    }} 
    pub type NotNullWhenAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.CodeAnalysis.NotNullWhenAttribute">;
    use super::super::super::*;
    impl From<NotNullWhenAttribute> for System::Attribute {
     fn from(v:NotNullWhenAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,NotNullWhenAttribute>(v)
    }} 
    pub type NotNullIfNotNullAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.CodeAnalysis.NotNullIfNotNullAttribute">;
    use super::super::super::*;
    impl From<NotNullIfNotNullAttribute> for System::Attribute {
     fn from(v:NotNullIfNotNullAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,NotNullIfNotNullAttribute>(v)
    }} 
    pub type DoesNotReturnAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.CodeAnalysis.DoesNotReturnAttribute">;
    use super::super::super::*;
    impl From<DoesNotReturnAttribute> for System::Attribute {
     fn from(v:DoesNotReturnAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,DoesNotReturnAttribute>(v)
    }} 
    pub type DoesNotReturnIfAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.CodeAnalysis.DoesNotReturnIfAttribute">;
    use super::super::super::*;
    impl From<DoesNotReturnIfAttribute> for System::Attribute {
     fn from(v:DoesNotReturnIfAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,DoesNotReturnIfAttribute>(v)
    }} 
    pub type MemberNotNullAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.CodeAnalysis.MemberNotNullAttribute">;
    use super::super::super::*;
    impl From<MemberNotNullAttribute> for System::Attribute {
     fn from(v:MemberNotNullAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,MemberNotNullAttribute>(v)
    }} 
    pub type MemberNotNullWhenAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.CodeAnalysis.MemberNotNullWhenAttribute">;
    use super::super::super::*;
    impl From<MemberNotNullWhenAttribute> for System::Attribute {
     fn from(v:MemberNotNullWhenAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,MemberNotNullWhenAttribute>(v)
    }} 
    pub type UnscopedRefAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.CodeAnalysis.UnscopedRefAttribute">;
    use super::super::super::*;
    impl From<UnscopedRefAttribute> for System::Attribute {
     fn from(v:UnscopedRefAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,UnscopedRefAttribute>(v)
    }} 
    pub type RequiresAssemblyFilesAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.CodeAnalysis.RequiresAssemblyFilesAttribute">;
    use super::super::super::*;
    impl From<RequiresAssemblyFilesAttribute> for System::Attribute {
     fn from(v:RequiresAssemblyFilesAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,RequiresAssemblyFilesAttribute>(v)
    }} 
    pub type RequiresDynamicCodeAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.CodeAnalysis.RequiresDynamicCodeAttribute">;
    use super::super::super::*;
    impl From<RequiresDynamicCodeAttribute> for System::Attribute {
     fn from(v:RequiresDynamicCodeAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,RequiresDynamicCodeAttribute>(v)
    }} 
    pub type RequiresUnreferencedCodeAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.CodeAnalysis.RequiresUnreferencedCodeAttribute">;
    use super::super::super::*;
    impl From<RequiresUnreferencedCodeAttribute> for System::Attribute {
     fn from(v:RequiresUnreferencedCodeAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,RequiresUnreferencedCodeAttribute>(v)
    }} 
    pub type SetsRequiredMembersAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.CodeAnalysis.SetsRequiredMembersAttribute">;
    use super::super::super::*;
    impl From<SetsRequiredMembersAttribute> for System::Attribute {
     fn from(v:SetsRequiredMembersAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,SetsRequiredMembersAttribute>(v)
    }} 
    pub type StringSyntaxAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.CodeAnalysis.StringSyntaxAttribute">;
    use super::super::super::*;
    impl From<StringSyntaxAttribute> for System::Attribute {
     fn from(v:StringSyntaxAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,StringSyntaxAttribute>(v)
    }} 
    pub type SuppressMessageAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.CodeAnalysis.SuppressMessageAttribute">;
    use super::super::super::*;
    impl From<SuppressMessageAttribute> for System::Attribute {
     fn from(v:SuppressMessageAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,SuppressMessageAttribute>(v)
    }} 
    pub type UnconditionalSuppressMessageAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.CodeAnalysis.UnconditionalSuppressMessageAttribute">;
    use super::super::super::*;
    impl From<UnconditionalSuppressMessageAttribute> for System::Attribute {
     fn from(v:UnconditionalSuppressMessageAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,UnconditionalSuppressMessageAttribute>(v)
    }} 
    }
    pub mod Contracts{
    pub type ContractException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Contracts.ContractException">;
    use super::super::super::*;
    impl From<ContractException> for System::Exception {
     fn from(v:ContractException)->System::Exception{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Exception,ContractException>(v)
    }} 
    pub type ContractFailedEventArgs =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Contracts.ContractFailedEventArgs">;
    use super::super::super::*;
    impl From<ContractFailedEventArgs> for System::EventArgs {
     fn from(v:ContractFailedEventArgs)->System::EventArgs{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::EventArgs,ContractFailedEventArgs>(v)
    }} 
    pub type PureAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Contracts.PureAttribute">;
    use super::super::super::*;
    impl From<PureAttribute> for System::Attribute {
     fn from(v:PureAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,PureAttribute>(v)
    }} 
    pub type ContractClassAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Contracts.ContractClassAttribute">;
    use super::super::super::*;
    impl From<ContractClassAttribute> for System::Attribute {
     fn from(v:ContractClassAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,ContractClassAttribute>(v)
    }} 
    pub type ContractClassForAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Contracts.ContractClassForAttribute">;
    use super::super::super::*;
    impl From<ContractClassForAttribute> for System::Attribute {
     fn from(v:ContractClassForAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,ContractClassForAttribute>(v)
    }} 
    pub type ContractInvariantMethodAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Contracts.ContractInvariantMethodAttribute">;
    use super::super::super::*;
    impl From<ContractInvariantMethodAttribute> for System::Attribute {
     fn from(v:ContractInvariantMethodAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,ContractInvariantMethodAttribute>(v)
    }} 
    pub type ContractReferenceAssemblyAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Contracts.ContractReferenceAssemblyAttribute">;
    use super::super::super::*;
    impl From<ContractReferenceAssemblyAttribute> for System::Attribute {
     fn from(v:ContractReferenceAssemblyAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,ContractReferenceAssemblyAttribute>(v)
    }} 
    pub type ContractRuntimeIgnoredAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Contracts.ContractRuntimeIgnoredAttribute">;
    use super::super::super::*;
    impl From<ContractRuntimeIgnoredAttribute> for System::Attribute {
     fn from(v:ContractRuntimeIgnoredAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,ContractRuntimeIgnoredAttribute>(v)
    }} 
    pub type ContractVerificationAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Contracts.ContractVerificationAttribute">;
    use super::super::super::*;
    impl From<ContractVerificationAttribute> for System::Attribute {
     fn from(v:ContractVerificationAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,ContractVerificationAttribute>(v)
    }} 
    pub type ContractPublicPropertyNameAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Contracts.ContractPublicPropertyNameAttribute">;
    use super::super::super::*;
    impl From<ContractPublicPropertyNameAttribute> for System::Attribute {
     fn from(v:ContractPublicPropertyNameAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,ContractPublicPropertyNameAttribute>(v)
    }} 
    pub type ContractArgumentValidatorAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Contracts.ContractArgumentValidatorAttribute">;
    use super::super::super::*;
    impl From<ContractArgumentValidatorAttribute> for System::Attribute {
     fn from(v:ContractArgumentValidatorAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,ContractArgumentValidatorAttribute>(v)
    }} 
    pub type ContractAbbreviatorAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Contracts.ContractAbbreviatorAttribute">;
    use super::super::super::*;
    impl From<ContractAbbreviatorAttribute> for System::Attribute {
     fn from(v:ContractAbbreviatorAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,ContractAbbreviatorAttribute>(v)
    }} 
    pub type ContractOptionAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Contracts.ContractOptionAttribute">;
    use super::super::super::*;
    impl From<ContractOptionAttribute> for System::Attribute {
     fn from(v:ContractOptionAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,ContractOptionAttribute>(v)
    }} 
    pub type Contract =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Contracts.Contract">;
    use super::super::super::*;
    impl From<Contract> for System::Object {
     fn from(v:Contract)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Contract>(v)
    }} 
    }
    pub type Debugger =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Debugger">;
    use super::super::*;
    impl From<Debugger> for System::Object {
     fn from(v:Debugger)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Debugger>(v)
    }} 
    pub type EditAndContinueHelper =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.EditAndContinueHelper">;
    use super::super::*;
    impl From<EditAndContinueHelper> for System::Object {
     fn from(v:EditAndContinueHelper)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,EditAndContinueHelper>(v)
    }} 
    pub type ICustomDebuggerNotification =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.ICustomDebuggerNotification">;
    use super::super::*;
    pub type StackFrame =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.StackFrame">;
    use super::super::*;
    impl From<StackFrame> for System::Object {
     fn from(v:StackFrame)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,StackFrame>(v)
    }} 
    pub type StackFrameHelper =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.StackFrameHelper">;
    use super::super::*;
    impl From<StackFrameHelper> for System::Object {
     fn from(v:StackFrameHelper)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,StackFrameHelper>(v)
    }} 
    pub type StackTrace =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.StackTrace">;
    use super::super::*;
    impl From<StackTrace> for System::Object {
     fn from(v:StackTrace)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,StackTrace>(v)
    }} 
    pub type ConditionalAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.ConditionalAttribute">;
    use super::super::*;
    impl From<ConditionalAttribute> for System::Attribute {
     fn from(v:ConditionalAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,ConditionalAttribute>(v)
    }} 
    pub type Debug =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Debug">;
    use super::super::*;
    impl From<Debug> for System::Object {
     fn from(v:Debug)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Debug>(v)
    }} 
    pub type DebuggableAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.DebuggableAttribute">;
    use super::super::*;
    impl From<DebuggableAttribute> for System::Attribute {
     fn from(v:DebuggableAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,DebuggableAttribute>(v)
    }} 
    pub type DebuggerBrowsableAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.DebuggerBrowsableAttribute">;
    use super::super::*;
    impl From<DebuggerBrowsableAttribute> for System::Attribute {
     fn from(v:DebuggerBrowsableAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,DebuggerBrowsableAttribute>(v)
    }} 
    pub type DebuggerDisplayAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.DebuggerDisplayAttribute">;
    use super::super::*;
    impl From<DebuggerDisplayAttribute> for System::Attribute {
     fn from(v:DebuggerDisplayAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,DebuggerDisplayAttribute>(v)
    }} 
    pub type DebuggerHiddenAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.DebuggerHiddenAttribute">;
    use super::super::*;
    impl From<DebuggerHiddenAttribute> for System::Attribute {
     fn from(v:DebuggerHiddenAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,DebuggerHiddenAttribute>(v)
    }} 
    pub type DebuggerNonUserCodeAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.DebuggerNonUserCodeAttribute">;
    use super::super::*;
    impl From<DebuggerNonUserCodeAttribute> for System::Attribute {
     fn from(v:DebuggerNonUserCodeAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,DebuggerNonUserCodeAttribute>(v)
    }} 
    pub type DebuggerStepperBoundaryAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.DebuggerStepperBoundaryAttribute">;
    use super::super::*;
    impl From<DebuggerStepperBoundaryAttribute> for System::Attribute {
     fn from(v:DebuggerStepperBoundaryAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,DebuggerStepperBoundaryAttribute>(v)
    }} 
    pub type DebuggerStepThroughAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.DebuggerStepThroughAttribute">;
    use super::super::*;
    impl From<DebuggerStepThroughAttribute> for System::Attribute {
     fn from(v:DebuggerStepThroughAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,DebuggerStepThroughAttribute>(v)
    }} 
    pub type DebuggerTypeProxyAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.DebuggerTypeProxyAttribute">;
    use super::super::*;
    impl From<DebuggerTypeProxyAttribute> for System::Attribute {
     fn from(v:DebuggerTypeProxyAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,DebuggerTypeProxyAttribute>(v)
    }} 
    pub type DebuggerVisualizerAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.DebuggerVisualizerAttribute">;
    use super::super::*;
    impl From<DebuggerVisualizerAttribute> for System::Attribute {
     fn from(v:DebuggerVisualizerAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,DebuggerVisualizerAttribute>(v)
    }} 
    pub type DebugProvider =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.DebugProvider">;
    use super::super::*;
    impl From<DebugProvider> for System::Object {
     fn from(v:DebugProvider)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,DebugProvider>(v)
    }} 
    pub type StackFrameExtensions =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.StackFrameExtensions">;
    use super::super::*;
    impl From<StackFrameExtensions> for System::Object {
     fn from(v:StackFrameExtensions)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,StackFrameExtensions>(v)
    }} 
    pub type StackTraceHiddenAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.StackTraceHiddenAttribute">;
    use super::super::*;
    impl From<StackTraceHiddenAttribute> for System::Attribute {
     fn from(v:StackTraceHiddenAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,StackTraceHiddenAttribute>(v)
    }} 
    pub type Stopwatch =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.Stopwatch">;
    use super::super::*;
    impl From<Stopwatch> for System::Object {
     fn from(v:Stopwatch)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Stopwatch>(v)
    }} 
    pub type UnreachableException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Diagnostics.UnreachableException">;
    use super::super::*;
    impl From<UnreachableException> for System::Exception {
     fn from(v:UnreachableException)->System::Exception{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Exception,UnreachableException>(v)
    }} 
    }
    pub mod Runtime{
    pub mod ConstrainedExecution{
    pub type CriticalFinalizerObject =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.ConstrainedExecution.CriticalFinalizerObject">;
    use super::super::super::*;
    impl From<CriticalFinalizerObject> for System::Object {
     fn from(v:CriticalFinalizerObject)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,CriticalFinalizerObject>(v)
    }} 
    pub type PrePrepareMethodAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.ConstrainedExecution.PrePrepareMethodAttribute">;
    use super::super::super::*;
    impl From<PrePrepareMethodAttribute> for System::Attribute {
     fn from(v:PrePrepareMethodAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,PrePrepareMethodAttribute>(v)
    }} 
    pub type ReliabilityContractAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.ConstrainedExecution.ReliabilityContractAttribute">;
    use super::super::super::*;
    impl From<ReliabilityContractAttribute> for System::Attribute {
     fn from(v:ReliabilityContractAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,ReliabilityContractAttribute>(v)
    }} 
    }
    pub mod Intrinsics{
    pub mod X86{
    pub type X86Base =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Intrinsics.X86.X86Base">;
    use super::super::super::super::*;
    impl From<X86Base> for System::Object {
     fn from(v:X86Base)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,X86Base>(v)
    }} 
    pub type Aes =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Intrinsics.X86.Aes">;
    use super::super::super::super::*;
    impl From<Aes> for System::Runtime::Intrinsics::X86::Sse2 {
     fn from(v:Aes)->System::Runtime::Intrinsics::X86::Sse2{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Runtime::Intrinsics::X86::Sse2,Aes>(v)
    }} 
    pub type Avx =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Intrinsics.X86.Avx">;
    use super::super::super::super::*;
    impl From<Avx> for System::Runtime::Intrinsics::X86::Sse42 {
     fn from(v:Avx)->System::Runtime::Intrinsics::X86::Sse42{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Runtime::Intrinsics::X86::Sse42,Avx>(v)
    }} 
    pub type Avx2 =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Intrinsics.X86.Avx2">;
    use super::super::super::super::*;
    impl From<Avx2> for System::Runtime::Intrinsics::X86::Avx {
     fn from(v:Avx2)->System::Runtime::Intrinsics::X86::Avx{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Runtime::Intrinsics::X86::Avx,Avx2>(v)
    }} 
    pub type Avx512BW =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Intrinsics.X86.Avx512BW">;
    use super::super::super::super::*;
    impl From<Avx512BW> for System::Runtime::Intrinsics::X86::Avx512F {
     fn from(v:Avx512BW)->System::Runtime::Intrinsics::X86::Avx512F{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Runtime::Intrinsics::X86::Avx512F,Avx512BW>(v)
    }} 
    pub type Avx512CD =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Intrinsics.X86.Avx512CD">;
    use super::super::super::super::*;
    impl From<Avx512CD> for System::Runtime::Intrinsics::X86::Avx512F {
     fn from(v:Avx512CD)->System::Runtime::Intrinsics::X86::Avx512F{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Runtime::Intrinsics::X86::Avx512F,Avx512CD>(v)
    }} 
    pub type Avx512DQ =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Intrinsics.X86.Avx512DQ">;
    use super::super::super::super::*;
    impl From<Avx512DQ> for System::Runtime::Intrinsics::X86::Avx512F {
     fn from(v:Avx512DQ)->System::Runtime::Intrinsics::X86::Avx512F{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Runtime::Intrinsics::X86::Avx512F,Avx512DQ>(v)
    }} 
    pub type Avx512F =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Intrinsics.X86.Avx512F">;
    use super::super::super::super::*;
    impl From<Avx512F> for System::Runtime::Intrinsics::X86::Avx2 {
     fn from(v:Avx512F)->System::Runtime::Intrinsics::X86::Avx2{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Runtime::Intrinsics::X86::Avx2,Avx512F>(v)
    }} 
    pub type Avx512Vbmi =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Intrinsics.X86.Avx512Vbmi">;
    use super::super::super::super::*;
    impl From<Avx512Vbmi> for System::Runtime::Intrinsics::X86::Avx512BW {
     fn from(v:Avx512Vbmi)->System::Runtime::Intrinsics::X86::Avx512BW{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Runtime::Intrinsics::X86::Avx512BW,Avx512Vbmi>(v)
    }} 
    pub type AvxVnni =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Intrinsics.X86.AvxVnni">;
    use super::super::super::super::*;
    impl From<AvxVnni> for System::Runtime::Intrinsics::X86::Avx2 {
     fn from(v:AvxVnni)->System::Runtime::Intrinsics::X86::Avx2{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Runtime::Intrinsics::X86::Avx2,AvxVnni>(v)
    }} 
    pub type Bmi1 =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Intrinsics.X86.Bmi1">;
    use super::super::super::super::*;
    impl From<Bmi1> for System::Runtime::Intrinsics::X86::X86Base {
     fn from(v:Bmi1)->System::Runtime::Intrinsics::X86::X86Base{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Runtime::Intrinsics::X86::X86Base,Bmi1>(v)
    }} 
    pub type Bmi2 =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Intrinsics.X86.Bmi2">;
    use super::super::super::super::*;
    impl From<Bmi2> for System::Runtime::Intrinsics::X86::X86Base {
     fn from(v:Bmi2)->System::Runtime::Intrinsics::X86::X86Base{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Runtime::Intrinsics::X86::X86Base,Bmi2>(v)
    }} 
    pub type Fma =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Intrinsics.X86.Fma">;
    use super::super::super::super::*;
    impl From<Fma> for System::Runtime::Intrinsics::X86::Avx {
     fn from(v:Fma)->System::Runtime::Intrinsics::X86::Avx{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Runtime::Intrinsics::X86::Avx,Fma>(v)
    }} 
    pub type Lzcnt =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Intrinsics.X86.Lzcnt">;
    use super::super::super::super::*;
    impl From<Lzcnt> for System::Runtime::Intrinsics::X86::X86Base {
     fn from(v:Lzcnt)->System::Runtime::Intrinsics::X86::X86Base{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Runtime::Intrinsics::X86::X86Base,Lzcnt>(v)
    }} 
    pub type Pclmulqdq =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Intrinsics.X86.Pclmulqdq">;
    use super::super::super::super::*;
    impl From<Pclmulqdq> for System::Runtime::Intrinsics::X86::Sse2 {
     fn from(v:Pclmulqdq)->System::Runtime::Intrinsics::X86::Sse2{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Runtime::Intrinsics::X86::Sse2,Pclmulqdq>(v)
    }} 
    pub type Popcnt =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Intrinsics.X86.Popcnt">;
    use super::super::super::super::*;
    impl From<Popcnt> for System::Runtime::Intrinsics::X86::Sse42 {
     fn from(v:Popcnt)->System::Runtime::Intrinsics::X86::Sse42{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Runtime::Intrinsics::X86::Sse42,Popcnt>(v)
    }} 
    pub type Sse =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Intrinsics.X86.Sse">;
    use super::super::super::super::*;
    impl From<Sse> for System::Runtime::Intrinsics::X86::X86Base {
     fn from(v:Sse)->System::Runtime::Intrinsics::X86::X86Base{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Runtime::Intrinsics::X86::X86Base,Sse>(v)
    }} 
    pub type Sse2 =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Intrinsics.X86.Sse2">;
    use super::super::super::super::*;
    impl From<Sse2> for System::Runtime::Intrinsics::X86::Sse {
     fn from(v:Sse2)->System::Runtime::Intrinsics::X86::Sse{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Runtime::Intrinsics::X86::Sse,Sse2>(v)
    }} 
    pub type Sse3 =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Intrinsics.X86.Sse3">;
    use super::super::super::super::*;
    impl From<Sse3> for System::Runtime::Intrinsics::X86::Sse2 {
     fn from(v:Sse3)->System::Runtime::Intrinsics::X86::Sse2{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Runtime::Intrinsics::X86::Sse2,Sse3>(v)
    }} 
    pub type Sse41 =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Intrinsics.X86.Sse41">;
    use super::super::super::super::*;
    impl From<Sse41> for System::Runtime::Intrinsics::X86::Ssse3 {
     fn from(v:Sse41)->System::Runtime::Intrinsics::X86::Ssse3{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Runtime::Intrinsics::X86::Ssse3,Sse41>(v)
    }} 
    pub type Sse42 =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Intrinsics.X86.Sse42">;
    use super::super::super::super::*;
    impl From<Sse42> for System::Runtime::Intrinsics::X86::Sse41 {
     fn from(v:Sse42)->System::Runtime::Intrinsics::X86::Sse41{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Runtime::Intrinsics::X86::Sse41,Sse42>(v)
    }} 
    pub type Ssse3 =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Intrinsics.X86.Ssse3">;
    use super::super::super::super::*;
    impl From<Ssse3> for System::Runtime::Intrinsics::X86::Sse3 {
     fn from(v:Ssse3)->System::Runtime::Intrinsics::X86::Sse3{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Runtime::Intrinsics::X86::Sse3,Ssse3>(v)
    }} 
    pub type X86Serialize =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Intrinsics.X86.X86Serialize">;
    use super::super::super::super::*;
    impl From<X86Serialize> for System::Runtime::Intrinsics::X86::X86Base {
     fn from(v:X86Serialize)->System::Runtime::Intrinsics::X86::X86Base{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Runtime::Intrinsics::X86::X86Base,X86Serialize>(v)
    }} 
    }
    pub mod Arm{
    pub type AdvSimd =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Intrinsics.Arm.AdvSimd">;
    use super::super::super::super::*;
    impl From<AdvSimd> for System::Runtime::Intrinsics::Arm::ArmBase {
     fn from(v:AdvSimd)->System::Runtime::Intrinsics::Arm::ArmBase{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Runtime::Intrinsics::Arm::ArmBase,AdvSimd>(v)
    }} 
    pub type Aes =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Intrinsics.Arm.Aes">;
    use super::super::super::super::*;
    impl From<Aes> for System::Runtime::Intrinsics::Arm::ArmBase {
     fn from(v:Aes)->System::Runtime::Intrinsics::Arm::ArmBase{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Runtime::Intrinsics::Arm::ArmBase,Aes>(v)
    }} 
    pub type ArmBase =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Intrinsics.Arm.ArmBase">;
    use super::super::super::super::*;
    impl From<ArmBase> for System::Object {
     fn from(v:ArmBase)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,ArmBase>(v)
    }} 
    pub type Crc32 =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Intrinsics.Arm.Crc32">;
    use super::super::super::super::*;
    impl From<Crc32> for System::Runtime::Intrinsics::Arm::ArmBase {
     fn from(v:Crc32)->System::Runtime::Intrinsics::Arm::ArmBase{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Runtime::Intrinsics::Arm::ArmBase,Crc32>(v)
    }} 
    pub type Dp =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Intrinsics.Arm.Dp">;
    use super::super::super::super::*;
    impl From<Dp> for System::Runtime::Intrinsics::Arm::AdvSimd {
     fn from(v:Dp)->System::Runtime::Intrinsics::Arm::AdvSimd{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Runtime::Intrinsics::Arm::AdvSimd,Dp>(v)
    }} 
    pub type Rdm =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Intrinsics.Arm.Rdm">;
    use super::super::super::super::*;
    impl From<Rdm> for System::Runtime::Intrinsics::Arm::AdvSimd {
     fn from(v:Rdm)->System::Runtime::Intrinsics::Arm::AdvSimd{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Runtime::Intrinsics::Arm::AdvSimd,Rdm>(v)
    }} 
    pub type Sha1 =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Intrinsics.Arm.Sha1">;
    use super::super::super::super::*;
    impl From<Sha1> for System::Runtime::Intrinsics::Arm::ArmBase {
     fn from(v:Sha1)->System::Runtime::Intrinsics::Arm::ArmBase{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Runtime::Intrinsics::Arm::ArmBase,Sha1>(v)
    }} 
    pub type Sha256 =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Intrinsics.Arm.Sha256">;
    use super::super::super::super::*;
    impl From<Sha256> for System::Runtime::Intrinsics::Arm::ArmBase {
     fn from(v:Sha256)->System::Runtime::Intrinsics::Arm::ArmBase{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Runtime::Intrinsics::Arm::ArmBase,Sha256>(v)
    }} 
    }
    pub mod Wasm{
    pub type PackedSimd =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Intrinsics.Wasm.PackedSimd">;
    use super::super::super::super::*;
    impl From<PackedSimd> for System::Object {
     fn from(v:PackedSimd)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,PackedSimd>(v)
    }} 
    }
    pub type Vector128 =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Intrinsics.Vector128">;
    use super::super::super::*;
    impl From<Vector128> for System::Object {
     fn from(v:Vector128)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Vector128>(v)
    }} 
    pub type Vector256 =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Intrinsics.Vector256">;
    use super::super::super::*;
    impl From<Vector256> for System::Object {
     fn from(v:Vector256)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Vector256>(v)
    }} 
    pub type Vector512 =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Intrinsics.Vector512">;
    use super::super::super::*;
    impl From<Vector512> for System::Object {
     fn from(v:Vector512)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Vector512>(v)
    }} 
    pub type Vector64 =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Intrinsics.Vector64">;
    use super::super::super::*;
    impl From<Vector64> for System::Object {
     fn from(v:Vector64)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Vector64>(v)
    }} 
    }
    pub mod Serialization{
    pub type DeserializationTracker =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Serialization.DeserializationTracker">;
    use super::super::super::*;
    impl From<DeserializationTracker> for System::Object {
     fn from(v:DeserializationTracker)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,DeserializationTracker>(v)
    }} 
    pub type IDeserializationCallback =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Serialization.IDeserializationCallback">;
    use super::super::super::*;
    pub type IFormatterConverter =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Serialization.IFormatterConverter">;
    use super::super::super::*;
    pub type IObjectReference =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Serialization.IObjectReference">;
    use super::super::super::*;
    pub type ISafeSerializationData =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Serialization.ISafeSerializationData">;
    use super::super::super::*;
    pub type ISerializable =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Serialization.ISerializable">;
    use super::super::super::*;
    pub type OnDeserializedAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Serialization.OnDeserializedAttribute">;
    use super::super::super::*;
    impl From<OnDeserializedAttribute> for System::Attribute {
     fn from(v:OnDeserializedAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,OnDeserializedAttribute>(v)
    }} 
    pub type OnDeserializingAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Serialization.OnDeserializingAttribute">;
    use super::super::super::*;
    impl From<OnDeserializingAttribute> for System::Attribute {
     fn from(v:OnDeserializingAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,OnDeserializingAttribute>(v)
    }} 
    pub type OnSerializedAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Serialization.OnSerializedAttribute">;
    use super::super::super::*;
    impl From<OnSerializedAttribute> for System::Attribute {
     fn from(v:OnSerializedAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,OnSerializedAttribute>(v)
    }} 
    pub type OnSerializingAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Serialization.OnSerializingAttribute">;
    use super::super::super::*;
    impl From<OnSerializingAttribute> for System::Attribute {
     fn from(v:OnSerializingAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,OnSerializingAttribute>(v)
    }} 
    pub type OptionalFieldAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Serialization.OptionalFieldAttribute">;
    use super::super::super::*;
    impl From<OptionalFieldAttribute> for System::Attribute {
     fn from(v:OptionalFieldAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,OptionalFieldAttribute>(v)
    }} 
    pub type SafeSerializationEventArgs =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Serialization.SafeSerializationEventArgs">;
    use super::super::super::*;
    impl From<SafeSerializationEventArgs> for System::EventArgs {
     fn from(v:SafeSerializationEventArgs)->System::EventArgs{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::EventArgs,SafeSerializationEventArgs>(v)
    }} 
    pub type SerializationException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Serialization.SerializationException">;
    use super::super::super::*;
    impl From<SerializationException> for System::SystemException {
     fn from(v:SerializationException)->System::SystemException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::SystemException,SerializationException>(v)
    }} 
    pub type SerializationInfo =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Serialization.SerializationInfo">;
    use super::super::super::*;
    impl From<SerializationInfo> for System::Object {
     fn from(v:SerializationInfo)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,SerializationInfo>(v)
    }} 
    pub type SerializationInfoEnumerator =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Serialization.SerializationInfoEnumerator">;
    use super::super::super::*;
    impl From<SerializationInfoEnumerator> for System::Object {
     fn from(v:SerializationInfoEnumerator)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,SerializationInfoEnumerator>(v)
    }} 
    }
    pub mod Remoting{
    pub type ObjectHandle =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Remoting.ObjectHandle">;
    use super::super::super::*;
    impl From<ObjectHandle> for System::MarshalByRefObject {
     fn from(v:ObjectHandle)->System::MarshalByRefObject{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::MarshalByRefObject,ObjectHandle>(v)
    }} 
    }
    pub mod Versioning{
    pub type CompatibilitySwitch =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Versioning.CompatibilitySwitch">;
    use super::super::super::*;
    impl From<CompatibilitySwitch> for System::Object {
     fn from(v:CompatibilitySwitch)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,CompatibilitySwitch>(v)
    }} 
    pub type ComponentGuaranteesAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Versioning.ComponentGuaranteesAttribute">;
    use super::super::super::*;
    impl From<ComponentGuaranteesAttribute> for System::Attribute {
     fn from(v:ComponentGuaranteesAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,ComponentGuaranteesAttribute>(v)
    }} 
    pub type FrameworkName =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Versioning.FrameworkName">;
    use super::super::super::*;
    impl From<FrameworkName> for System::Object {
     fn from(v:FrameworkName)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,FrameworkName>(v)
    }} 
    pub type OSPlatformAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Versioning.OSPlatformAttribute">;
    use super::super::super::*;
    impl From<OSPlatformAttribute> for System::Attribute {
     fn from(v:OSPlatformAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,OSPlatformAttribute>(v)
    }} 
    pub type TargetPlatformAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Versioning.TargetPlatformAttribute">;
    use super::super::super::*;
    impl From<TargetPlatformAttribute> for System::Runtime::Versioning::OSPlatformAttribute {
     fn from(v:TargetPlatformAttribute)->System::Runtime::Versioning::OSPlatformAttribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Runtime::Versioning::OSPlatformAttribute,TargetPlatformAttribute>(v)
    }} 
    pub type SupportedOSPlatformAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Versioning.SupportedOSPlatformAttribute">;
    use super::super::super::*;
    impl From<SupportedOSPlatformAttribute> for System::Runtime::Versioning::OSPlatformAttribute {
     fn from(v:SupportedOSPlatformAttribute)->System::Runtime::Versioning::OSPlatformAttribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Runtime::Versioning::OSPlatformAttribute,SupportedOSPlatformAttribute>(v)
    }} 
    pub type UnsupportedOSPlatformAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Versioning.UnsupportedOSPlatformAttribute">;
    use super::super::super::*;
    impl From<UnsupportedOSPlatformAttribute> for System::Runtime::Versioning::OSPlatformAttribute {
     fn from(v:UnsupportedOSPlatformAttribute)->System::Runtime::Versioning::OSPlatformAttribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Runtime::Versioning::OSPlatformAttribute,UnsupportedOSPlatformAttribute>(v)
    }} 
    pub type ObsoletedOSPlatformAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Versioning.ObsoletedOSPlatformAttribute">;
    use super::super::super::*;
    impl From<ObsoletedOSPlatformAttribute> for System::Runtime::Versioning::OSPlatformAttribute {
     fn from(v:ObsoletedOSPlatformAttribute)->System::Runtime::Versioning::OSPlatformAttribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Runtime::Versioning::OSPlatformAttribute,ObsoletedOSPlatformAttribute>(v)
    }} 
    pub type SupportedOSPlatformGuardAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Versioning.SupportedOSPlatformGuardAttribute">;
    use super::super::super::*;
    impl From<SupportedOSPlatformGuardAttribute> for System::Runtime::Versioning::OSPlatformAttribute {
     fn from(v:SupportedOSPlatformGuardAttribute)->System::Runtime::Versioning::OSPlatformAttribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Runtime::Versioning::OSPlatformAttribute,SupportedOSPlatformGuardAttribute>(v)
    }} 
    pub type UnsupportedOSPlatformGuardAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Versioning.UnsupportedOSPlatformGuardAttribute">;
    use super::super::super::*;
    impl From<UnsupportedOSPlatformGuardAttribute> for System::Runtime::Versioning::OSPlatformAttribute {
     fn from(v:UnsupportedOSPlatformGuardAttribute)->System::Runtime::Versioning::OSPlatformAttribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Runtime::Versioning::OSPlatformAttribute,UnsupportedOSPlatformGuardAttribute>(v)
    }} 
    pub type RequiresPreviewFeaturesAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Versioning.RequiresPreviewFeaturesAttribute">;
    use super::super::super::*;
    impl From<RequiresPreviewFeaturesAttribute> for System::Attribute {
     fn from(v:RequiresPreviewFeaturesAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,RequiresPreviewFeaturesAttribute>(v)
    }} 
    pub type ResourceConsumptionAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Versioning.ResourceConsumptionAttribute">;
    use super::super::super::*;
    impl From<ResourceConsumptionAttribute> for System::Attribute {
     fn from(v:ResourceConsumptionAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,ResourceConsumptionAttribute>(v)
    }} 
    pub type ResourceExposureAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Versioning.ResourceExposureAttribute">;
    use super::super::super::*;
    impl From<ResourceExposureAttribute> for System::Attribute {
     fn from(v:ResourceExposureAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,ResourceExposureAttribute>(v)
    }} 
    pub type TargetFrameworkAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Versioning.TargetFrameworkAttribute">;
    use super::super::super::*;
    impl From<TargetFrameworkAttribute> for System::Attribute {
     fn from(v:TargetFrameworkAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,TargetFrameworkAttribute>(v)
    }} 
    pub type VersioningHelper =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Versioning.VersioningHelper">;
    use super::super::super::*;
    impl From<VersioningHelper> for System::Object {
     fn from(v:VersioningHelper)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,VersioningHelper>(v)
    }} 
    pub type NonVersionableAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Versioning.NonVersionableAttribute">;
    use super::super::super::*;
    impl From<NonVersionableAttribute> for System::Attribute {
     fn from(v:NonVersionableAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,NonVersionableAttribute>(v)
    }} 
    }
    pub mod ExceptionServices{
    pub type ExceptionDispatchInfo =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.ExceptionServices.ExceptionDispatchInfo">;
    use super::super::super::*;
    impl From<ExceptionDispatchInfo> for System::Object {
     fn from(v:ExceptionDispatchInfo)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,ExceptionDispatchInfo>(v)
    }} 
    pub type FirstChanceExceptionEventArgs =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.ExceptionServices.FirstChanceExceptionEventArgs">;
    use super::super::super::*;
    impl From<FirstChanceExceptionEventArgs> for System::EventArgs {
     fn from(v:FirstChanceExceptionEventArgs)->System::EventArgs{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::EventArgs,FirstChanceExceptionEventArgs>(v)
    }} 
    pub type HandleProcessCorruptedStateExceptionsAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.ExceptionServices.HandleProcessCorruptedStateExceptionsAttribute">;
    use super::super::super::*;
    impl From<HandleProcessCorruptedStateExceptionsAttribute> for System::Attribute {
     fn from(v:HandleProcessCorruptedStateExceptionsAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,HandleProcessCorruptedStateExceptionsAttribute>(v)
    }} 
    }
    pub mod CompilerServices{
    pub type CastHelpers =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.CastHelpers">;
    use super::super::super::*;
    impl From<CastHelpers> for System::Object {
     fn from(v:CastHelpers)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,CastHelpers>(v)
    }} 
    pub type ICastableHelpers =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.ICastableHelpers">;
    use super::super::super::*;
    impl From<ICastableHelpers> for System::Object {
     fn from(v:ICastableHelpers)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,ICastableHelpers>(v)
    }} 
    pub type RuntimeHelpers =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.RuntimeHelpers">;
    use super::super::super::*;
    impl From<RuntimeHelpers> for System::Object {
     fn from(v:RuntimeHelpers)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,RuntimeHelpers>(v)
    }} 
    pub type RawData =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.RawData">;
    use super::super::super::*;
    impl From<RawData> for System::Object {
     fn from(v:RawData)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,RawData>(v)
    }} 
    pub type RawArrayData =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.RawArrayData">;
    use super::super::super::*;
    impl From<RawArrayData> for System::Object {
     fn from(v:RawArrayData)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,RawArrayData>(v)
    }} 
    pub type AccessedThroughPropertyAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.AccessedThroughPropertyAttribute">;
    use super::super::super::*;
    impl From<AccessedThroughPropertyAttribute> for System::Attribute {
     fn from(v:AccessedThroughPropertyAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,AccessedThroughPropertyAttribute>(v)
    }} 
    pub type AsyncIteratorStateMachineAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.AsyncIteratorStateMachineAttribute">;
    use super::super::super::*;
    impl From<AsyncIteratorStateMachineAttribute> for System::Runtime::CompilerServices::StateMachineAttribute {
     fn from(v:AsyncIteratorStateMachineAttribute)->System::Runtime::CompilerServices::StateMachineAttribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Runtime::CompilerServices::StateMachineAttribute,AsyncIteratorStateMachineAttribute>(v)
    }} 
    pub type AsyncMethodBuilderAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.AsyncMethodBuilderAttribute">;
    use super::super::super::*;
    impl From<AsyncMethodBuilderAttribute> for System::Attribute {
     fn from(v:AsyncMethodBuilderAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,AsyncMethodBuilderAttribute>(v)
    }} 
    pub type AsyncMethodBuilderCore =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.AsyncMethodBuilderCore">;
    use super::super::super::*;
    impl From<AsyncMethodBuilderCore> for System::Object {
     fn from(v:AsyncMethodBuilderCore)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,AsyncMethodBuilderCore>(v)
    }} 
    pub type AsyncStateMachineAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.AsyncStateMachineAttribute">;
    use super::super::super::*;
    impl From<AsyncStateMachineAttribute> for System::Runtime::CompilerServices::StateMachineAttribute {
     fn from(v:AsyncStateMachineAttribute)->System::Runtime::CompilerServices::StateMachineAttribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Runtime::CompilerServices::StateMachineAttribute,AsyncStateMachineAttribute>(v)
    }} 
    pub type CallerArgumentExpressionAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.CallerArgumentExpressionAttribute">;
    use super::super::super::*;
    impl From<CallerArgumentExpressionAttribute> for System::Attribute {
     fn from(v:CallerArgumentExpressionAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,CallerArgumentExpressionAttribute>(v)
    }} 
    pub type CallerFilePathAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.CallerFilePathAttribute">;
    use super::super::super::*;
    impl From<CallerFilePathAttribute> for System::Attribute {
     fn from(v:CallerFilePathAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,CallerFilePathAttribute>(v)
    }} 
    pub type CallerLineNumberAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.CallerLineNumberAttribute">;
    use super::super::super::*;
    impl From<CallerLineNumberAttribute> for System::Attribute {
     fn from(v:CallerLineNumberAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,CallerLineNumberAttribute>(v)
    }} 
    pub type CallerMemberNameAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.CallerMemberNameAttribute">;
    use super::super::super::*;
    impl From<CallerMemberNameAttribute> for System::Attribute {
     fn from(v:CallerMemberNameAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,CallerMemberNameAttribute>(v)
    }} 
    pub type CallConvCdecl =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.CallConvCdecl">;
    use super::super::super::*;
    impl From<CallConvCdecl> for System::Object {
     fn from(v:CallConvCdecl)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,CallConvCdecl>(v)
    }} 
    pub type CallConvFastcall =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.CallConvFastcall">;
    use super::super::super::*;
    impl From<CallConvFastcall> for System::Object {
     fn from(v:CallConvFastcall)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,CallConvFastcall>(v)
    }} 
    pub type CallConvStdcall =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.CallConvStdcall">;
    use super::super::super::*;
    impl From<CallConvStdcall> for System::Object {
     fn from(v:CallConvStdcall)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,CallConvStdcall>(v)
    }} 
    pub type CallConvSuppressGCTransition =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.CallConvSuppressGCTransition">;
    use super::super::super::*;
    impl From<CallConvSuppressGCTransition> for System::Object {
     fn from(v:CallConvSuppressGCTransition)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,CallConvSuppressGCTransition>(v)
    }} 
    pub type CallConvThiscall =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.CallConvThiscall">;
    use super::super::super::*;
    impl From<CallConvThiscall> for System::Object {
     fn from(v:CallConvThiscall)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,CallConvThiscall>(v)
    }} 
    pub type CallConvMemberFunction =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.CallConvMemberFunction">;
    use super::super::super::*;
    impl From<CallConvMemberFunction> for System::Object {
     fn from(v:CallConvMemberFunction)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,CallConvMemberFunction>(v)
    }} 
    pub type CollectionBuilderAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.CollectionBuilderAttribute">;
    use super::super::super::*;
    impl From<CollectionBuilderAttribute> for System::Attribute {
     fn from(v:CollectionBuilderAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,CollectionBuilderAttribute>(v)
    }} 
    pub type CompExactlyDependsOnAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.CompExactlyDependsOnAttribute">;
    use super::super::super::*;
    impl From<CompExactlyDependsOnAttribute> for System::Attribute {
     fn from(v:CompExactlyDependsOnAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,CompExactlyDependsOnAttribute>(v)
    }} 
    pub type CompilationRelaxationsAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.CompilationRelaxationsAttribute">;
    use super::super::super::*;
    impl From<CompilationRelaxationsAttribute> for System::Attribute {
     fn from(v:CompilationRelaxationsAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,CompilationRelaxationsAttribute>(v)
    }} 
    pub type CompilerFeatureRequiredAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.CompilerFeatureRequiredAttribute">;
    use super::super::super::*;
    impl From<CompilerFeatureRequiredAttribute> for System::Attribute {
     fn from(v:CompilerFeatureRequiredAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,CompilerFeatureRequiredAttribute>(v)
    }} 
    pub type CompilerGeneratedAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.CompilerGeneratedAttribute">;
    use super::super::super::*;
    impl From<CompilerGeneratedAttribute> for System::Attribute {
     fn from(v:CompilerGeneratedAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,CompilerGeneratedAttribute>(v)
    }} 
    pub type CompilerGlobalScopeAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.CompilerGlobalScopeAttribute">;
    use super::super::super::*;
    impl From<CompilerGlobalScopeAttribute> for System::Attribute {
     fn from(v:CompilerGlobalScopeAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,CompilerGlobalScopeAttribute>(v)
    }} 
    pub type ContractHelper =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.ContractHelper">;
    use super::super::super::*;
    impl From<ContractHelper> for System::Object {
     fn from(v:ContractHelper)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,ContractHelper>(v)
    }} 
    pub type CreateNewOnMetadataUpdateAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.CreateNewOnMetadataUpdateAttribute">;
    use super::super::super::*;
    impl From<CreateNewOnMetadataUpdateAttribute> for System::Attribute {
     fn from(v:CreateNewOnMetadataUpdateAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,CreateNewOnMetadataUpdateAttribute>(v)
    }} 
    pub type CustomConstantAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.CustomConstantAttribute">;
    use super::super::super::*;
    impl From<CustomConstantAttribute> for System::Attribute {
     fn from(v:CustomConstantAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,CustomConstantAttribute>(v)
    }} 
    pub type DateTimeConstantAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.DateTimeConstantAttribute">;
    use super::super::super::*;
    impl From<DateTimeConstantAttribute> for System::Runtime::CompilerServices::CustomConstantAttribute {
     fn from(v:DateTimeConstantAttribute)->System::Runtime::CompilerServices::CustomConstantAttribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Runtime::CompilerServices::CustomConstantAttribute,DateTimeConstantAttribute>(v)
    }} 
    pub type DecimalConstantAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.DecimalConstantAttribute">;
    use super::super::super::*;
    impl From<DecimalConstantAttribute> for System::Attribute {
     fn from(v:DecimalConstantAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,DecimalConstantAttribute>(v)
    }} 
    pub type DefaultDependencyAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.DefaultDependencyAttribute">;
    use super::super::super::*;
    impl From<DefaultDependencyAttribute> for System::Attribute {
     fn from(v:DefaultDependencyAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,DefaultDependencyAttribute>(v)
    }} 
    pub type DependencyAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.DependencyAttribute">;
    use super::super::super::*;
    impl From<DependencyAttribute> for System::Attribute {
     fn from(v:DependencyAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,DependencyAttribute>(v)
    }} 
    pub type DisablePrivateReflectionAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.DisablePrivateReflectionAttribute">;
    use super::super::super::*;
    impl From<DisablePrivateReflectionAttribute> for System::Attribute {
     fn from(v:DisablePrivateReflectionAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,DisablePrivateReflectionAttribute>(v)
    }} 
    pub type DisableRuntimeMarshallingAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.DisableRuntimeMarshallingAttribute">;
    use super::super::super::*;
    impl From<DisableRuntimeMarshallingAttribute> for System::Attribute {
     fn from(v:DisableRuntimeMarshallingAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,DisableRuntimeMarshallingAttribute>(v)
    }} 
    pub type DiscardableAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.DiscardableAttribute">;
    use super::super::super::*;
    impl From<DiscardableAttribute> for System::Attribute {
     fn from(v:DiscardableAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,DiscardableAttribute>(v)
    }} 
    pub type EnumeratorCancellationAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.EnumeratorCancellationAttribute">;
    use super::super::super::*;
    impl From<EnumeratorCancellationAttribute> for System::Attribute {
     fn from(v:EnumeratorCancellationAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,EnumeratorCancellationAttribute>(v)
    }} 
    pub type ExtensionAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.ExtensionAttribute">;
    use super::super::super::*;
    impl From<ExtensionAttribute> for System::Attribute {
     fn from(v:ExtensionAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,ExtensionAttribute>(v)
    }} 
    pub type FixedAddressValueTypeAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.FixedAddressValueTypeAttribute">;
    use super::super::super::*;
    impl From<FixedAddressValueTypeAttribute> for System::Attribute {
     fn from(v:FixedAddressValueTypeAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,FixedAddressValueTypeAttribute>(v)
    }} 
    pub type FixedBufferAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.FixedBufferAttribute">;
    use super::super::super::*;
    impl From<FixedBufferAttribute> for System::Attribute {
     fn from(v:FixedBufferAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,FixedBufferAttribute>(v)
    }} 
    pub type FormattableStringFactory =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.FormattableStringFactory">;
    use super::super::super::*;
    impl From<FormattableStringFactory> for System::Object {
     fn from(v:FormattableStringFactory)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,FormattableStringFactory>(v)
    }} 
    pub type IAsyncStateMachine =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.IAsyncStateMachine">;
    use super::super::super::*;
    pub type IAsyncStateMachineBox =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.IAsyncStateMachineBox">;
    use super::super::super::*;
    pub type ICastable =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.ICastable">;
    use super::super::super::*;
    pub type IndexerNameAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.IndexerNameAttribute">;
    use super::super::super::*;
    impl From<IndexerNameAttribute> for System::Attribute {
     fn from(v:IndexerNameAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,IndexerNameAttribute>(v)
    }} 
    pub type INotifyCompletion =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.INotifyCompletion">;
    use super::super::super::*;
    pub type ICriticalNotifyCompletion =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.ICriticalNotifyCompletion">;
    use super::super::super::*;
    pub type InternalsVisibleToAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.InternalsVisibleToAttribute">;
    use super::super::super::*;
    impl From<InternalsVisibleToAttribute> for System::Attribute {
     fn from(v:InternalsVisibleToAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,InternalsVisibleToAttribute>(v)
    }} 
    pub type IntrinsicAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.IntrinsicAttribute">;
    use super::super::super::*;
    impl From<IntrinsicAttribute> for System::Attribute {
     fn from(v:IntrinsicAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,IntrinsicAttribute>(v)
    }} 
    pub type IsByRefLikeAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.IsByRefLikeAttribute">;
    use super::super::super::*;
    impl From<IsByRefLikeAttribute> for System::Attribute {
     fn from(v:IsByRefLikeAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,IsByRefLikeAttribute>(v)
    }} 
    pub type InlineArrayAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.InlineArrayAttribute">;
    use super::super::super::*;
    impl From<InlineArrayAttribute> for System::Attribute {
     fn from(v:InlineArrayAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,InlineArrayAttribute>(v)
    }} 
    pub type IsConst =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.IsConst">;
    use super::super::super::*;
    impl From<IsConst> for System::Object {
     fn from(v:IsConst)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,IsConst>(v)
    }} 
    pub type IsExternalInit =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.IsExternalInit">;
    use super::super::super::*;
    impl From<IsExternalInit> for System::Object {
     fn from(v:IsExternalInit)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,IsExternalInit>(v)
    }} 
    pub type IsReadOnlyAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.IsReadOnlyAttribute">;
    use super::super::super::*;
    impl From<IsReadOnlyAttribute> for System::Attribute {
     fn from(v:IsReadOnlyAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,IsReadOnlyAttribute>(v)
    }} 
    pub type IsVolatile =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.IsVolatile">;
    use super::super::super::*;
    impl From<IsVolatile> for System::Object {
     fn from(v:IsVolatile)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,IsVolatile>(v)
    }} 
    pub type InterpolatedStringHandlerAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.InterpolatedStringHandlerAttribute">;
    use super::super::super::*;
    impl From<InterpolatedStringHandlerAttribute> for System::Attribute {
     fn from(v:InterpolatedStringHandlerAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,InterpolatedStringHandlerAttribute>(v)
    }} 
    pub type InterpolatedStringHandlerArgumentAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.InterpolatedStringHandlerArgumentAttribute">;
    use super::super::super::*;
    impl From<InterpolatedStringHandlerArgumentAttribute> for System::Attribute {
     fn from(v:InterpolatedStringHandlerArgumentAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,InterpolatedStringHandlerArgumentAttribute>(v)
    }} 
    pub type IsUnmanagedAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.IsUnmanagedAttribute">;
    use super::super::super::*;
    impl From<IsUnmanagedAttribute> for System::Attribute {
     fn from(v:IsUnmanagedAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,IsUnmanagedAttribute>(v)
    }} 
    pub type IteratorStateMachineAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.IteratorStateMachineAttribute">;
    use super::super::super::*;
    impl From<IteratorStateMachineAttribute> for System::Runtime::CompilerServices::StateMachineAttribute {
     fn from(v:IteratorStateMachineAttribute)->System::Runtime::CompilerServices::StateMachineAttribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Runtime::CompilerServices::StateMachineAttribute,IteratorStateMachineAttribute>(v)
    }} 
    pub type ITuple =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.ITuple">;
    use super::super::super::*;
    pub type MethodImplAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.MethodImplAttribute">;
    use super::super::super::*;
    impl From<MethodImplAttribute> for System::Attribute {
     fn from(v:MethodImplAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,MethodImplAttribute>(v)
    }} 
    pub type ModuleInitializerAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.ModuleInitializerAttribute">;
    use super::super::super::*;
    impl From<ModuleInitializerAttribute> for System::Attribute {
     fn from(v:ModuleInitializerAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,ModuleInitializerAttribute>(v)
    }} 
    pub type MetadataUpdateOriginalTypeAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.MetadataUpdateOriginalTypeAttribute">;
    use super::super::super::*;
    impl From<MetadataUpdateOriginalTypeAttribute> for System::Attribute {
     fn from(v:MetadataUpdateOriginalTypeAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,MetadataUpdateOriginalTypeAttribute>(v)
    }} 
    pub type NullableAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.NullableAttribute">;
    use super::super::super::*;
    impl From<NullableAttribute> for System::Attribute {
     fn from(v:NullableAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,NullableAttribute>(v)
    }} 
    pub type NullableContextAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.NullableContextAttribute">;
    use super::super::super::*;
    impl From<NullableContextAttribute> for System::Attribute {
     fn from(v:NullableContextAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,NullableContextAttribute>(v)
    }} 
    pub type NullablePublicOnlyAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.NullablePublicOnlyAttribute">;
    use super::super::super::*;
    impl From<NullablePublicOnlyAttribute> for System::Attribute {
     fn from(v:NullablePublicOnlyAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,NullablePublicOnlyAttribute>(v)
    }} 
    pub type ReferenceAssemblyAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.ReferenceAssemblyAttribute">;
    use super::super::super::*;
    impl From<ReferenceAssemblyAttribute> for System::Attribute {
     fn from(v:ReferenceAssemblyAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,ReferenceAssemblyAttribute>(v)
    }} 
    pub type PreserveBaseOverridesAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.PreserveBaseOverridesAttribute">;
    use super::super::super::*;
    impl From<PreserveBaseOverridesAttribute> for System::Attribute {
     fn from(v:PreserveBaseOverridesAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,PreserveBaseOverridesAttribute>(v)
    }} 
    pub type RefSafetyRulesAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.RefSafetyRulesAttribute">;
    use super::super::super::*;
    impl From<RefSafetyRulesAttribute> for System::Attribute {
     fn from(v:RefSafetyRulesAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,RefSafetyRulesAttribute>(v)
    }} 
    pub type RequiredMemberAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.RequiredMemberAttribute">;
    use super::super::super::*;
    impl From<RequiredMemberAttribute> for System::Attribute {
     fn from(v:RequiredMemberAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,RequiredMemberAttribute>(v)
    }} 
    pub type RequiresLocationAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.RequiresLocationAttribute">;
    use super::super::super::*;
    impl From<RequiresLocationAttribute> for System::Attribute {
     fn from(v:RequiresLocationAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,RequiresLocationAttribute>(v)
    }} 
    pub type RuntimeCompatibilityAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.RuntimeCompatibilityAttribute">;
    use super::super::super::*;
    impl From<RuntimeCompatibilityAttribute> for System::Attribute {
     fn from(v:RuntimeCompatibilityAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,RuntimeCompatibilityAttribute>(v)
    }} 
    pub type RuntimeFeature =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.RuntimeFeature">;
    use super::super::super::*;
    impl From<RuntimeFeature> for System::Object {
     fn from(v:RuntimeFeature)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,RuntimeFeature>(v)
    }} 
    pub type RuntimeWrappedException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.RuntimeWrappedException">;
    use super::super::super::*;
    impl From<RuntimeWrappedException> for System::Exception {
     fn from(v:RuntimeWrappedException)->System::Exception{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Exception,RuntimeWrappedException>(v)
    }} 
    pub type ScopedRefAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.ScopedRefAttribute">;
    use super::super::super::*;
    impl From<ScopedRefAttribute> for System::Attribute {
     fn from(v:ScopedRefAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,ScopedRefAttribute>(v)
    }} 
    pub type SkipLocalsInitAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.SkipLocalsInitAttribute">;
    use super::super::super::*;
    impl From<SkipLocalsInitAttribute> for System::Attribute {
     fn from(v:SkipLocalsInitAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,SkipLocalsInitAttribute>(v)
    }} 
    pub type SpecialNameAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.SpecialNameAttribute">;
    use super::super::super::*;
    impl From<SpecialNameAttribute> for System::Attribute {
     fn from(v:SpecialNameAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,SpecialNameAttribute>(v)
    }} 
    pub type StateMachineAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.StateMachineAttribute">;
    use super::super::super::*;
    impl From<StateMachineAttribute> for System::Attribute {
     fn from(v:StateMachineAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,StateMachineAttribute>(v)
    }} 
    pub type StringFreezingAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.StringFreezingAttribute">;
    use super::super::super::*;
    impl From<StringFreezingAttribute> for System::Attribute {
     fn from(v:StringFreezingAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,StringFreezingAttribute>(v)
    }} 
    pub type IStrongBox =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.IStrongBox">;
    use super::super::super::*;
    pub type SuppressIldasmAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.SuppressIldasmAttribute">;
    use super::super::super::*;
    impl From<SuppressIldasmAttribute> for System::Attribute {
     fn from(v:SuppressIldasmAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,SuppressIldasmAttribute>(v)
    }} 
    pub type SwitchExpressionException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.SwitchExpressionException">;
    use super::super::super::*;
    impl From<SwitchExpressionException> for System::InvalidOperationException {
     fn from(v:SwitchExpressionException)->System::InvalidOperationException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::InvalidOperationException,SwitchExpressionException>(v)
    }} 
    pub type ITaskAwaiter =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.ITaskAwaiter">;
    use super::super::super::*;
    pub type IConfiguredTaskAwaiter =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.IConfiguredTaskAwaiter">;
    use super::super::super::*;
    pub type TupleElementNamesAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.TupleElementNamesAttribute">;
    use super::super::super::*;
    impl From<TupleElementNamesAttribute> for System::Attribute {
     fn from(v:TupleElementNamesAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,TupleElementNamesAttribute>(v)
    }} 
    pub type TypeForwardedFromAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.TypeForwardedFromAttribute">;
    use super::super::super::*;
    impl From<TypeForwardedFromAttribute> for System::Attribute {
     fn from(v:TypeForwardedFromAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,TypeForwardedFromAttribute>(v)
    }} 
    pub type TypeForwardedToAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.TypeForwardedToAttribute">;
    use super::super::super::*;
    impl From<TypeForwardedToAttribute> for System::Attribute {
     fn from(v:TypeForwardedToAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,TypeForwardedToAttribute>(v)
    }} 
    pub type Unsafe =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.Unsafe">;
    use super::super::super::*;
    impl From<Unsafe> for System::Object {
     fn from(v:Unsafe)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Unsafe>(v)
    }} 
    pub type UnsafeAccessorAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.UnsafeAccessorAttribute">;
    use super::super::super::*;
    impl From<UnsafeAccessorAttribute> for System::Attribute {
     fn from(v:UnsafeAccessorAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,UnsafeAccessorAttribute>(v)
    }} 
    pub type UnsafeValueTypeAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.UnsafeValueTypeAttribute">;
    use super::super::super::*;
    impl From<UnsafeValueTypeAttribute> for System::Attribute {
     fn from(v:UnsafeValueTypeAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,UnsafeValueTypeAttribute>(v)
    }} 
    pub type IStateMachineBoxAwareAwaiter =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.CompilerServices.IStateMachineBoxAwareAwaiter">;
    use super::super::super::*;
    }
    pub mod Loader{
    pub type AssemblyLoadContext =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Loader.AssemblyLoadContext">;
    use super::super::super::*;
    impl From<AssemblyLoadContext> for System::Object {
     fn from(v:AssemblyLoadContext)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,AssemblyLoadContext>(v)
    }} 
    pub type DefaultAssemblyLoadContext =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Loader.DefaultAssemblyLoadContext">;
    use super::super::super::*;
    impl From<DefaultAssemblyLoadContext> for System::Runtime::Loader::AssemblyLoadContext {
     fn from(v:DefaultAssemblyLoadContext)->System::Runtime::Loader::AssemblyLoadContext{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Runtime::Loader::AssemblyLoadContext,DefaultAssemblyLoadContext>(v)
    }} 
    pub type IndividualAssemblyLoadContext =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Loader.IndividualAssemblyLoadContext">;
    use super::super::super::*;
    impl From<IndividualAssemblyLoadContext> for System::Runtime::Loader::AssemblyLoadContext {
     fn from(v:IndividualAssemblyLoadContext)->System::Runtime::Loader::AssemblyLoadContext{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Runtime::Loader::AssemblyLoadContext,IndividualAssemblyLoadContext>(v)
    }} 
    pub type AssemblyDependencyResolver =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.Loader.AssemblyDependencyResolver">;
    use super::super::super::*;
    impl From<AssemblyDependencyResolver> for System::Object {
     fn from(v:AssemblyDependencyResolver)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,AssemblyDependencyResolver>(v)
    }} 
    }
    pub mod InteropServices{
    pub mod Marshalling{
    pub type AnsiStringMarshaller =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.Marshalling.AnsiStringMarshaller">;
    use super::super::super::super::*;
    impl From<AnsiStringMarshaller> for System::Object {
     fn from(v:AnsiStringMarshaller)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,AnsiStringMarshaller>(v)
    }} 
    pub type BStrStringMarshaller =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.Marshalling.BStrStringMarshaller">;
    use super::super::super::super::*;
    impl From<BStrStringMarshaller> for System::Object {
     fn from(v:BStrStringMarshaller)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,BStrStringMarshaller>(v)
    }} 
    pub type ContiguousCollectionMarshallerAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.Marshalling.ContiguousCollectionMarshallerAttribute">;
    use super::super::super::super::*;
    impl From<ContiguousCollectionMarshallerAttribute> for System::Attribute {
     fn from(v:ContiguousCollectionMarshallerAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,ContiguousCollectionMarshallerAttribute>(v)
    }} 
    pub type CustomMarshallerAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.Marshalling.CustomMarshallerAttribute">;
    use super::super::super::super::*;
    impl From<CustomMarshallerAttribute> for System::Attribute {
     fn from(v:CustomMarshallerAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,CustomMarshallerAttribute>(v)
    }} 
    pub type MarshalUsingAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.Marshalling.MarshalUsingAttribute">;
    use super::super::super::super::*;
    impl From<MarshalUsingAttribute> for System::Attribute {
     fn from(v:MarshalUsingAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,MarshalUsingAttribute>(v)
    }} 
    pub type NativeMarshallingAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.Marshalling.NativeMarshallingAttribute">;
    use super::super::super::super::*;
    impl From<NativeMarshallingAttribute> for System::Attribute {
     fn from(v:NativeMarshallingAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,NativeMarshallingAttribute>(v)
    }} 
    pub type Utf16StringMarshaller =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.Marshalling.Utf16StringMarshaller">;
    use super::super::super::super::*;
    impl From<Utf16StringMarshaller> for System::Object {
     fn from(v:Utf16StringMarshaller)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Utf16StringMarshaller>(v)
    }} 
    pub type Utf8StringMarshaller =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.Marshalling.Utf8StringMarshaller">;
    use super::super::super::super::*;
    impl From<Utf8StringMarshaller> for System::Object {
     fn from(v:Utf8StringMarshaller)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Utf8StringMarshaller>(v)
    }} 
    }
    pub mod ComTypes{
    pub type IEnumerable =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.ComTypes.IEnumerable">;
    use super::super::super::super::*;
    pub type IEnumerator =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.ComTypes.IEnumerator">;
    use super::super::super::super::*;
    pub type IBindCtx =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.ComTypes.IBindCtx">;
    use super::super::super::super::*;
    pub type IConnectionPoint =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.ComTypes.IConnectionPoint">;
    use super::super::super::super::*;
    pub type IConnectionPointContainer =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.ComTypes.IConnectionPointContainer">;
    use super::super::super::super::*;
    pub type IEnumConnectionPoints =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.ComTypes.IEnumConnectionPoints">;
    use super::super::super::super::*;
    pub type IEnumConnections =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.ComTypes.IEnumConnections">;
    use super::super::super::super::*;
    pub type IEnumMoniker =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.ComTypes.IEnumMoniker">;
    use super::super::super::super::*;
    pub type IEnumString =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.ComTypes.IEnumString">;
    use super::super::super::super::*;
    pub type IEnumVARIANT =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.ComTypes.IEnumVARIANT">;
    use super::super::super::super::*;
    pub type IMoniker =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.ComTypes.IMoniker">;
    use super::super::super::super::*;
    pub type IPersistFile =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.ComTypes.IPersistFile">;
    use super::super::super::super::*;
    pub type IRunningObjectTable =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.ComTypes.IRunningObjectTable">;
    use super::super::super::super::*;
    pub type IStream =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.ComTypes.IStream">;
    use super::super::super::super::*;
    pub type ITypeComp =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.ComTypes.ITypeComp">;
    use super::super::super::super::*;
    pub type ITypeInfo =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.ComTypes.ITypeInfo">;
    use super::super::super::super::*;
    pub type ITypeInfo2 =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.ComTypes.ITypeInfo2">;
    use super::super::super::super::*;
    pub type ITypeLib =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.ComTypes.ITypeLib">;
    use super::super::super::super::*;
    pub type ITypeLib2 =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.ComTypes.ITypeLib2">;
    use super::super::super::super::*;
    }
    pub mod ObjectiveC{
    pub type ObjectiveCMarshal =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.ObjectiveC.ObjectiveCMarshal">;
    use super::super::super::super::*;
    impl From<ObjectiveCMarshal> for System::Object {
     fn from(v:ObjectiveCMarshal)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,ObjectiveCMarshal>(v)
    }} 
    pub type ObjectiveCTrackedTypeAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.ObjectiveC.ObjectiveCTrackedTypeAttribute">;
    use super::super::super::super::*;
    impl From<ObjectiveCTrackedTypeAttribute> for System::Attribute {
     fn from(v:ObjectiveCTrackedTypeAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,ObjectiveCTrackedTypeAttribute>(v)
    }} 
    }
    pub type DynamicInterfaceCastableHelpers =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.DynamicInterfaceCastableHelpers">;
    use super::super::super::*;
    impl From<DynamicInterfaceCastableHelpers> for System::Object {
     fn from(v:DynamicInterfaceCastableHelpers)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,DynamicInterfaceCastableHelpers>(v)
    }} 
    pub type Marshal =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.Marshal">;
    use super::super::super::*;
    impl From<Marshal> for System::Object {
     fn from(v:Marshal)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Marshal>(v)
    }} 
    pub type MemoryMarshal =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.MemoryMarshal">;
    use super::super::super::*;
    impl From<MemoryMarshal> for System::Object {
     fn from(v:MemoryMarshal)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,MemoryMarshal>(v)
    }} 
    pub type NativeLibrary =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.NativeLibrary">;
    use super::super::super::*;
    impl From<NativeLibrary> for System::Object {
     fn from(v:NativeLibrary)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,NativeLibrary>(v)
    }} 
    pub type ComWrappers =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.ComWrappers">;
    use super::super::super::*;
    impl From<ComWrappers> for System::Object {
     fn from(v:ComWrappers)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,ComWrappers>(v)
    }} 
    pub type AllowReversePInvokeCallsAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.AllowReversePInvokeCallsAttribute">;
    use super::super::super::*;
    impl From<AllowReversePInvokeCallsAttribute> for System::Attribute {
     fn from(v:AllowReversePInvokeCallsAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,AllowReversePInvokeCallsAttribute>(v)
    }} 
    pub type BestFitMappingAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.BestFitMappingAttribute">;
    use super::super::super::*;
    impl From<BestFitMappingAttribute> for System::Attribute {
     fn from(v:BestFitMappingAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,BestFitMappingAttribute>(v)
    }} 
    pub type BStrWrapper =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.BStrWrapper">;
    use super::super::super::*;
    impl From<BStrWrapper> for System::Object {
     fn from(v:BStrWrapper)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,BStrWrapper>(v)
    }} 
    pub type ClassInterfaceAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.ClassInterfaceAttribute">;
    use super::super::super::*;
    impl From<ClassInterfaceAttribute> for System::Attribute {
     fn from(v:ClassInterfaceAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,ClassInterfaceAttribute>(v)
    }} 
    pub type CoClassAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.CoClassAttribute">;
    use super::super::super::*;
    impl From<CoClassAttribute> for System::Attribute {
     fn from(v:CoClassAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,CoClassAttribute>(v)
    }} 
    pub type CollectionsMarshal =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.CollectionsMarshal">;
    use super::super::super::*;
    impl From<CollectionsMarshal> for System::Object {
     fn from(v:CollectionsMarshal)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,CollectionsMarshal>(v)
    }} 
    pub type ComDefaultInterfaceAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.ComDefaultInterfaceAttribute">;
    use super::super::super::*;
    impl From<ComDefaultInterfaceAttribute> for System::Attribute {
     fn from(v:ComDefaultInterfaceAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,ComDefaultInterfaceAttribute>(v)
    }} 
    pub type ComEventInterfaceAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.ComEventInterfaceAttribute">;
    use super::super::super::*;
    impl From<ComEventInterfaceAttribute> for System::Attribute {
     fn from(v:ComEventInterfaceAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,ComEventInterfaceAttribute>(v)
    }} 
    pub type COMException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.COMException">;
    use super::super::super::*;
    impl From<COMException> for System::Runtime::InteropServices::ExternalException {
     fn from(v:COMException)->System::Runtime::InteropServices::ExternalException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Runtime::InteropServices::ExternalException,COMException>(v)
    }} 
    pub type ComImportAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.ComImportAttribute">;
    use super::super::super::*;
    impl From<ComImportAttribute> for System::Attribute {
     fn from(v:ComImportAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,ComImportAttribute>(v)
    }} 
    pub type ComSourceInterfacesAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.ComSourceInterfacesAttribute">;
    use super::super::super::*;
    impl From<ComSourceInterfacesAttribute> for System::Attribute {
     fn from(v:ComSourceInterfacesAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,ComSourceInterfacesAttribute>(v)
    }} 
    pub type ComVisibleAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.ComVisibleAttribute">;
    use super::super::super::*;
    impl From<ComVisibleAttribute> for System::Attribute {
     fn from(v:ComVisibleAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,ComVisibleAttribute>(v)
    }} 
    pub type CriticalHandle =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.CriticalHandle">;
    use super::super::super::*;
    impl From<CriticalHandle> for System::Runtime::ConstrainedExecution::CriticalFinalizerObject {
     fn from(v:CriticalHandle)->System::Runtime::ConstrainedExecution::CriticalFinalizerObject{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Runtime::ConstrainedExecution::CriticalFinalizerObject,CriticalHandle>(v)
    }} 
    pub type CurrencyWrapper =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.CurrencyWrapper">;
    use super::super::super::*;
    impl From<CurrencyWrapper> for System::Object {
     fn from(v:CurrencyWrapper)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,CurrencyWrapper>(v)
    }} 
    pub type DefaultCharSetAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.DefaultCharSetAttribute">;
    use super::super::super::*;
    impl From<DefaultCharSetAttribute> for System::Attribute {
     fn from(v:DefaultCharSetAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,DefaultCharSetAttribute>(v)
    }} 
    pub type DefaultDllImportSearchPathsAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.DefaultDllImportSearchPathsAttribute">;
    use super::super::super::*;
    impl From<DefaultDllImportSearchPathsAttribute> for System::Attribute {
     fn from(v:DefaultDllImportSearchPathsAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,DefaultDllImportSearchPathsAttribute>(v)
    }} 
    pub type DefaultParameterValueAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.DefaultParameterValueAttribute">;
    use super::super::super::*;
    impl From<DefaultParameterValueAttribute> for System::Attribute {
     fn from(v:DefaultParameterValueAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,DefaultParameterValueAttribute>(v)
    }} 
    pub type DispatchWrapper =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.DispatchWrapper">;
    use super::super::super::*;
    impl From<DispatchWrapper> for System::Object {
     fn from(v:DispatchWrapper)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,DispatchWrapper>(v)
    }} 
    pub type DispIdAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.DispIdAttribute">;
    use super::super::super::*;
    impl From<DispIdAttribute> for System::Attribute {
     fn from(v:DispIdAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,DispIdAttribute>(v)
    }} 
    pub type DllImportAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.DllImportAttribute">;
    use super::super::super::*;
    impl From<DllImportAttribute> for System::Attribute {
     fn from(v:DllImportAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,DllImportAttribute>(v)
    }} 
    pub type ErrorWrapper =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.ErrorWrapper">;
    use super::super::super::*;
    impl From<ErrorWrapper> for System::Object {
     fn from(v:ErrorWrapper)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,ErrorWrapper>(v)
    }} 
    pub type ExternalException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.ExternalException">;
    use super::super::super::*;
    impl From<ExternalException> for System::SystemException {
     fn from(v:ExternalException)->System::SystemException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::SystemException,ExternalException>(v)
    }} 
    pub type FieldOffsetAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.FieldOffsetAttribute">;
    use super::super::super::*;
    impl From<FieldOffsetAttribute> for System::Attribute {
     fn from(v:FieldOffsetAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,FieldOffsetAttribute>(v)
    }} 
    pub type GuidAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.GuidAttribute">;
    use super::super::super::*;
    impl From<GuidAttribute> for System::Attribute {
     fn from(v:GuidAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,GuidAttribute>(v)
    }} 
    pub type ICustomAdapter =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.ICustomAdapter">;
    use super::super::super::*;
    pub type ICustomFactory =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.ICustomFactory">;
    use super::super::super::*;
    pub type ICustomMarshaler =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.ICustomMarshaler">;
    use super::super::super::*;
    pub type ICustomQueryInterface =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.ICustomQueryInterface">;
    use super::super::super::*;
    pub type IDynamicInterfaceCastable =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.IDynamicInterfaceCastable">;
    use super::super::super::*;
    pub type DynamicInterfaceCastableImplementationAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.DynamicInterfaceCastableImplementationAttribute">;
    use super::super::super::*;
    impl From<DynamicInterfaceCastableImplementationAttribute> for System::Attribute {
     fn from(v:DynamicInterfaceCastableImplementationAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,DynamicInterfaceCastableImplementationAttribute>(v)
    }} 
    pub type InAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.InAttribute">;
    use super::super::super::*;
    impl From<InAttribute> for System::Attribute {
     fn from(v:InAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,InAttribute>(v)
    }} 
    pub type InterfaceTypeAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.InterfaceTypeAttribute">;
    use super::super::super::*;
    impl From<InterfaceTypeAttribute> for System::Attribute {
     fn from(v:InterfaceTypeAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,InterfaceTypeAttribute>(v)
    }} 
    pub type InvalidComObjectException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.InvalidComObjectException">;
    use super::super::super::*;
    impl From<InvalidComObjectException> for System::SystemException {
     fn from(v:InvalidComObjectException)->System::SystemException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::SystemException,InvalidComObjectException>(v)
    }} 
    pub type InvalidOleVariantTypeException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.InvalidOleVariantTypeException">;
    use super::super::super::*;
    impl From<InvalidOleVariantTypeException> for System::SystemException {
     fn from(v:InvalidOleVariantTypeException)->System::SystemException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::SystemException,InvalidOleVariantTypeException>(v)
    }} 
    pub type LCIDConversionAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.LCIDConversionAttribute">;
    use super::super::super::*;
    impl From<LCIDConversionAttribute> for System::Attribute {
     fn from(v:LCIDConversionAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,LCIDConversionAttribute>(v)
    }} 
    pub type LibraryImportAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.LibraryImportAttribute">;
    use super::super::super::*;
    impl From<LibraryImportAttribute> for System::Attribute {
     fn from(v:LibraryImportAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,LibraryImportAttribute>(v)
    }} 
    pub type MarshalAsAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.MarshalAsAttribute">;
    use super::super::super::*;
    impl From<MarshalAsAttribute> for System::Attribute {
     fn from(v:MarshalAsAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,MarshalAsAttribute>(v)
    }} 
    pub type MarshalDirectiveException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.MarshalDirectiveException">;
    use super::super::super::*;
    impl From<MarshalDirectiveException> for System::SystemException {
     fn from(v:MarshalDirectiveException)->System::SystemException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::SystemException,MarshalDirectiveException>(v)
    }} 
    pub type DllImportResolver =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.DllImportResolver">;
    use super::super::super::*;
    impl From<DllImportResolver> for System::MulticastDelegate {
     fn from(v:DllImportResolver)->System::MulticastDelegate{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::MulticastDelegate,DllImportResolver>(v)
    }} 
    pub type NativeMemory =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.NativeMemory">;
    use super::super::super::*;
    impl From<NativeMemory> for System::Object {
     fn from(v:NativeMemory)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,NativeMemory>(v)
    }} 
    pub type OptionalAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.OptionalAttribute">;
    use super::super::super::*;
    impl From<OptionalAttribute> for System::Attribute {
     fn from(v:OptionalAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,OptionalAttribute>(v)
    }} 
    pub type OutAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.OutAttribute">;
    use super::super::super::*;
    impl From<OutAttribute> for System::Attribute {
     fn from(v:OutAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,OutAttribute>(v)
    }} 
    pub type PosixSignalContext =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.PosixSignalContext">;
    use super::super::super::*;
    impl From<PosixSignalContext> for System::Object {
     fn from(v:PosixSignalContext)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,PosixSignalContext>(v)
    }} 
    pub type PosixSignalRegistration =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.PosixSignalRegistration">;
    use super::super::super::*;
    impl From<PosixSignalRegistration> for System::Object {
     fn from(v:PosixSignalRegistration)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,PosixSignalRegistration>(v)
    }} 
    pub type PreserveSigAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.PreserveSigAttribute">;
    use super::super::super::*;
    impl From<PreserveSigAttribute> for System::Attribute {
     fn from(v:PreserveSigAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,PreserveSigAttribute>(v)
    }} 
    pub type ProgIdAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.ProgIdAttribute">;
    use super::super::super::*;
    impl From<ProgIdAttribute> for System::Attribute {
     fn from(v:ProgIdAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,ProgIdAttribute>(v)
    }} 
    pub type RuntimeInformation =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.RuntimeInformation">;
    use super::super::super::*;
    impl From<RuntimeInformation> for System::Object {
     fn from(v:RuntimeInformation)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,RuntimeInformation>(v)
    }} 
    pub type SafeArrayRankMismatchException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.SafeArrayRankMismatchException">;
    use super::super::super::*;
    impl From<SafeArrayRankMismatchException> for System::SystemException {
     fn from(v:SafeArrayRankMismatchException)->System::SystemException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::SystemException,SafeArrayRankMismatchException>(v)
    }} 
    pub type SafeArrayTypeMismatchException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.SafeArrayTypeMismatchException">;
    use super::super::super::*;
    impl From<SafeArrayTypeMismatchException> for System::SystemException {
     fn from(v:SafeArrayTypeMismatchException)->System::SystemException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::SystemException,SafeArrayTypeMismatchException>(v)
    }} 
    pub type SafeBuffer =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.SafeBuffer">;
    use super::super::super::*;
    impl From<SafeBuffer> for Microsoft::Win32::SafeHandles::SafeHandleZeroOrMinusOneIsInvalid {
     fn from(v:SafeBuffer)->Microsoft::Win32::SafeHandles::SafeHandleZeroOrMinusOneIsInvalid{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<Microsoft::Win32::SafeHandles::SafeHandleZeroOrMinusOneIsInvalid,SafeBuffer>(v)
    }} 
    pub type SafeHandle =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.SafeHandle">;
    use super::super::super::*;
    impl From<SafeHandle> for System::Runtime::ConstrainedExecution::CriticalFinalizerObject {
     fn from(v:SafeHandle)->System::Runtime::ConstrainedExecution::CriticalFinalizerObject{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Runtime::ConstrainedExecution::CriticalFinalizerObject,SafeHandle>(v)
    }} 
    pub type SEHException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.SEHException">;
    use super::super::super::*;
    impl From<SEHException> for System::Runtime::InteropServices::ExternalException {
     fn from(v:SEHException)->System::Runtime::InteropServices::ExternalException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Runtime::InteropServices::ExternalException,SEHException>(v)
    }} 
    pub type StructLayoutAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.StructLayoutAttribute">;
    use super::super::super::*;
    impl From<StructLayoutAttribute> for System::Attribute {
     fn from(v:StructLayoutAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,StructLayoutAttribute>(v)
    }} 
    pub type SuppressGCTransitionAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.SuppressGCTransitionAttribute">;
    use super::super::super::*;
    impl From<SuppressGCTransitionAttribute> for System::Attribute {
     fn from(v:SuppressGCTransitionAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,SuppressGCTransitionAttribute>(v)
    }} 
    pub type TypeIdentifierAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.TypeIdentifierAttribute">;
    use super::super::super::*;
    impl From<TypeIdentifierAttribute> for System::Attribute {
     fn from(v:TypeIdentifierAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,TypeIdentifierAttribute>(v)
    }} 
    pub type UnknownWrapper =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.UnknownWrapper">;
    use super::super::super::*;
    impl From<UnknownWrapper> for System::Object {
     fn from(v:UnknownWrapper)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,UnknownWrapper>(v)
    }} 
    pub type UnmanagedCallConvAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.UnmanagedCallConvAttribute">;
    use super::super::super::*;
    impl From<UnmanagedCallConvAttribute> for System::Attribute {
     fn from(v:UnmanagedCallConvAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,UnmanagedCallConvAttribute>(v)
    }} 
    pub type UnmanagedCallersOnlyAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.UnmanagedCallersOnlyAttribute">;
    use super::super::super::*;
    impl From<UnmanagedCallersOnlyAttribute> for System::Attribute {
     fn from(v:UnmanagedCallersOnlyAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,UnmanagedCallersOnlyAttribute>(v)
    }} 
    pub type UnmanagedFunctionPointerAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.UnmanagedFunctionPointerAttribute">;
    use super::super::super::*;
    impl From<UnmanagedFunctionPointerAttribute> for System::Attribute {
     fn from(v:UnmanagedFunctionPointerAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,UnmanagedFunctionPointerAttribute>(v)
    }} 
    pub type VariantWrapper =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.VariantWrapper">;
    use super::super::super::*;
    impl From<VariantWrapper> for System::Object {
     fn from(v:VariantWrapper)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,VariantWrapper>(v)
    }} 
    pub type ComEventsHelper =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.ComEventsHelper">;
    use super::super::super::*;
    impl From<ComEventsHelper> for System::Object {
     fn from(v:ComEventsHelper)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,ComEventsHelper>(v)
    }} 
    pub type StandardOleMarshalObject =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.InteropServices.StandardOleMarshalObject">;
    use super::super::super::*;
    impl From<StandardOleMarshalObject> for System::MarshalByRefObject {
     fn from(v:StandardOleMarshalObject)->System::MarshalByRefObject{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::MarshalByRefObject,StandardOleMarshalObject>(v)
    }} 
    }
    pub type ControlledExecution =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.ControlledExecution">;
    use super::super::*;
    impl From<ControlledExecution> for System::Object {
     fn from(v:ControlledExecution)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,ControlledExecution>(v)
    }} 
    pub type GCSettings =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.GCSettings">;
    use super::super::*;
    impl From<GCSettings> for System::Object {
     fn from(v:GCSettings)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,GCSettings>(v)
    }} 
    pub type JitInfo =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.JitInfo">;
    use super::super::*;
    impl From<JitInfo> for System::Object {
     fn from(v:JitInfo)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,JitInfo>(v)
    }} 
    pub type AmbiguousImplementationException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.AmbiguousImplementationException">;
    use super::super::*;
    impl From<AmbiguousImplementationException> for System::Exception {
     fn from(v:AmbiguousImplementationException)->System::Exception{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Exception,AmbiguousImplementationException>(v)
    }} 
    pub type MemoryFailPoint =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.MemoryFailPoint">;
    use super::super::*;
    impl From<MemoryFailPoint> for System::Runtime::ConstrainedExecution::CriticalFinalizerObject {
     fn from(v:MemoryFailPoint)->System::Runtime::ConstrainedExecution::CriticalFinalizerObject{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Runtime::ConstrainedExecution::CriticalFinalizerObject,MemoryFailPoint>(v)
    }} 
    pub type AssemblyTargetedPatchBandAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.AssemblyTargetedPatchBandAttribute">;
    use super::super::*;
    impl From<AssemblyTargetedPatchBandAttribute> for System::Attribute {
     fn from(v:AssemblyTargetedPatchBandAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,AssemblyTargetedPatchBandAttribute>(v)
    }} 
    pub type TargetedPatchingOptOutAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.TargetedPatchingOptOutAttribute">;
    use super::super::*;
    impl From<TargetedPatchingOptOutAttribute> for System::Attribute {
     fn from(v:TargetedPatchingOptOutAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,TargetedPatchingOptOutAttribute>(v)
    }} 
    pub type ProfileOptimization =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Runtime.ProfileOptimization">;
    use super::super::*;
    impl From<ProfileOptimization> for System::Object {
     fn from(v:ProfileOptimization)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,ProfileOptimization>(v)
    }} 
    }
    pub mod Collections{
    pub mod Generic{
    pub type ComparerHelpers =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Collections.Generic.ComparerHelpers">;
    use super::super::super::*;
    impl From<ComparerHelpers> for System::Object {
     fn from(v:ComparerHelpers)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,ComparerHelpers>(v)
    }} 
    pub type ByteEqualityComparer =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Collections.Generic.ByteEqualityComparer">;
    use super::super::super::*;
    pub type SortUtils =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Collections.Generic.SortUtils">;
    use super::super::super::*;
    impl From<SortUtils> for System::Object {
     fn from(v:SortUtils)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,SortUtils>(v)
    }} 
    pub type CollectionExtensions =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Collections.Generic.CollectionExtensions">;
    use super::super::super::*;
    impl From<CollectionExtensions> for System::Object {
     fn from(v:CollectionExtensions)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,CollectionExtensions>(v)
    }} 
    pub type IInternalStringEqualityComparer =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Collections.Generic.IInternalStringEqualityComparer">;
    use super::super::super::*;
    pub type KeyNotFoundException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Collections.Generic.KeyNotFoundException">;
    use super::super::super::*;
    impl From<KeyNotFoundException> for System::SystemException {
     fn from(v:KeyNotFoundException)->System::SystemException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::SystemException,KeyNotFoundException>(v)
    }} 
    pub type KeyValuePair =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Collections.Generic.KeyValuePair">;
    use super::super::super::*;
    impl From<KeyValuePair> for System::Object {
     fn from(v:KeyValuePair)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,KeyValuePair>(v)
    }} 
    pub type RandomizedStringEqualityComparer =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Collections.Generic.RandomizedStringEqualityComparer">;
    use super::super::super::*;
    pub type ReferenceEqualityComparer =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Collections.Generic.ReferenceEqualityComparer">;
    use super::super::super::*;
    impl From<ReferenceEqualityComparer> for System::Object {
     fn from(v:ReferenceEqualityComparer)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,ReferenceEqualityComparer>(v)
    }} 
    pub type NonRandomizedStringEqualityComparer =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Collections.Generic.NonRandomizedStringEqualityComparer">;
    use super::super::super::*;
    impl From<NonRandomizedStringEqualityComparer> for System::Object {
     fn from(v:NonRandomizedStringEqualityComparer)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,NonRandomizedStringEqualityComparer>(v)
    }} 
    pub type EnumerableHelpers =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Collections.Generic.EnumerableHelpers">;
    use super::super::super::*;
    impl From<EnumerableHelpers> for System::Object {
     fn from(v:EnumerableHelpers)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,EnumerableHelpers>(v)
    }} 
    }
    pub mod ObjectModel{
    pub type CollectionHelpers =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Collections.ObjectModel.CollectionHelpers">;
    use super::super::super::*;
    impl From<CollectionHelpers> for System::Object {
     fn from(v:CollectionHelpers)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,CollectionHelpers>(v)
    }} 
    }
    pub mod Concurrent{
    }
    pub type EmptyReadOnlyDictionaryInternal =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Collections.EmptyReadOnlyDictionaryInternal">;
    use super::super::*;
    impl From<EmptyReadOnlyDictionaryInternal> for System::Object {
     fn from(v:EmptyReadOnlyDictionaryInternal)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,EmptyReadOnlyDictionaryInternal>(v)
    }} 
    pub type ArrayList =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Collections.ArrayList">;
    use super::super::*;
    impl From<ArrayList> for System::Object {
     fn from(v:ArrayList)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,ArrayList>(v)
    }} 
    pub type Comparer =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Collections.Comparer">;
    use super::super::*;
    impl From<Comparer> for System::Object {
     fn from(v:Comparer)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Comparer>(v)
    }} 
    pub type CompatibleComparer =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Collections.CompatibleComparer">;
    use super::super::*;
    impl From<CompatibleComparer> for System::Object {
     fn from(v:CompatibleComparer)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,CompatibleComparer>(v)
    }} 
    pub type HashHelpers =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Collections.HashHelpers">;
    use super::super::*;
    impl From<HashHelpers> for System::Object {
     fn from(v:HashHelpers)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,HashHelpers>(v)
    }} 
    pub type Hashtable =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Collections.Hashtable">;
    use super::super::*;
    impl From<Hashtable> for System::Object {
     fn from(v:Hashtable)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Hashtable>(v)
    }} 
    pub type ICollection =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Collections.ICollection">;
    use super::super::*;
    pub type IComparer =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Collections.IComparer">;
    use super::super::*;
    pub type IDictionary =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Collections.IDictionary">;
    use super::super::*;
    pub type IDictionaryEnumerator =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Collections.IDictionaryEnumerator">;
    use super::super::*;
    pub type IEnumerable =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Collections.IEnumerable">;
    use super::super::*;
    pub type IEnumerator =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Collections.IEnumerator">;
    use super::super::*;
    pub type IEqualityComparer =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Collections.IEqualityComparer">;
    use super::super::*;
    pub type IHashCodeProvider =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Collections.IHashCodeProvider">;
    use super::super::*;
    pub type IList =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Collections.IList">;
    use super::super::*;
    pub type IStructuralComparable =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Collections.IStructuralComparable">;
    use super::super::*;
    pub type IStructuralEquatable =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Collections.IStructuralEquatable">;
    use super::super::*;
    pub type KeyValuePairs =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Collections.KeyValuePairs">;
    use super::super::*;
    impl From<KeyValuePairs> for System::Object {
     fn from(v:KeyValuePairs)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,KeyValuePairs>(v)
    }} 
    pub type ListDictionaryInternal =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Collections.ListDictionaryInternal">;
    use super::super::*;
    impl From<ListDictionaryInternal> for System::Object {
     fn from(v:ListDictionaryInternal)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,ListDictionaryInternal>(v)
    }} 
    }
    pub mod Threading{
    pub mod Tasks{
    pub mod Sources{
    pub type IValueTaskSource =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.Tasks.Sources.IValueTaskSource">;
    use super::super::super::super::*;
    pub type CapturedSchedulerAndExecutionContext =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.Tasks.Sources.CapturedSchedulerAndExecutionContext">;
    use super::super::super::super::*;
    impl From<CapturedSchedulerAndExecutionContext> for System::Object {
     fn from(v:CapturedSchedulerAndExecutionContext)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,CapturedSchedulerAndExecutionContext>(v)
    }} 
    pub type ManualResetValueTaskSourceCoreShared =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.Tasks.Sources.ManualResetValueTaskSourceCoreShared">;
    use super::super::super::super::*;
    impl From<ManualResetValueTaskSourceCoreShared> for System::Object {
     fn from(v:ManualResetValueTaskSourceCoreShared)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,ManualResetValueTaskSourceCoreShared>(v)
    }} 
    }
    pub type ConcurrentExclusiveSchedulerPair =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.Tasks.ConcurrentExclusiveSchedulerPair">;
    use super::super::super::*;
    impl From<ConcurrentExclusiveSchedulerPair> for System::Object {
     fn from(v:ConcurrentExclusiveSchedulerPair)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,ConcurrentExclusiveSchedulerPair>(v)
    }} 
    pub type Task =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.Tasks.Task">;
    use super::super::super::*;
    impl From<Task> for System::Object {
     fn from(v:Task)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Task>(v)
    }} 
    pub type CompletionActionInvoker =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.Tasks.CompletionActionInvoker">;
    use super::super::super::*;
    impl From<CompletionActionInvoker> for System::Object {
     fn from(v:CompletionActionInvoker)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,CompletionActionInvoker>(v)
    }} 
    pub type SystemThreadingTasks_TaskDebugView =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.Tasks.SystemThreadingTasks_TaskDebugView">;
    use super::super::super::*;
    impl From<SystemThreadingTasks_TaskDebugView> for System::Object {
     fn from(v:SystemThreadingTasks_TaskDebugView)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,SystemThreadingTasks_TaskDebugView>(v)
    }} 
    pub type ITaskCompletionAction =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.Tasks.ITaskCompletionAction">;
    use super::super::super::*;
    pub type TaskAsyncEnumerableExtensions =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.Tasks.TaskAsyncEnumerableExtensions">;
    use super::super::super::*;
    impl From<TaskAsyncEnumerableExtensions> for System::Object {
     fn from(v:TaskAsyncEnumerableExtensions)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,TaskAsyncEnumerableExtensions>(v)
    }} 
    pub type TaskCache =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.Tasks.TaskCache">;
    use super::super::super::*;
    impl From<TaskCache> for System::Object {
     fn from(v:TaskCache)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,TaskCache>(v)
    }} 
    pub type TaskCanceledException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.Tasks.TaskCanceledException">;
    use super::super::super::*;
    impl From<TaskCanceledException> for System::OperationCanceledException {
     fn from(v:TaskCanceledException)->System::OperationCanceledException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::OperationCanceledException,TaskCanceledException>(v)
    }} 
    pub type TaskCompletionSource =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.Tasks.TaskCompletionSource">;
    use super::super::super::*;
    impl From<TaskCompletionSource> for System::Object {
     fn from(v:TaskCompletionSource)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,TaskCompletionSource>(v)
    }} 
    pub type ContinuationTaskFromTask =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.Tasks.ContinuationTaskFromTask">;
    use super::super::super::*;
    impl From<ContinuationTaskFromTask> for System::Threading::Tasks::Task {
     fn from(v:ContinuationTaskFromTask)->System::Threading::Tasks::Task{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Threading::Tasks::Task,ContinuationTaskFromTask>(v)
    }} 
    pub type TaskContinuation =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.Tasks.TaskContinuation">;
    use super::super::super::*;
    impl From<TaskContinuation> for System::Object {
     fn from(v:TaskContinuation)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,TaskContinuation>(v)
    }} 
    pub type ContinueWithTaskContinuation =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.Tasks.ContinueWithTaskContinuation">;
    use super::super::super::*;
    impl From<ContinueWithTaskContinuation> for System::Threading::Tasks::TaskContinuation {
     fn from(v:ContinueWithTaskContinuation)->System::Threading::Tasks::TaskContinuation{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Threading::Tasks::TaskContinuation,ContinueWithTaskContinuation>(v)
    }} 
    pub type SynchronizationContextAwaitTaskContinuation =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.Tasks.SynchronizationContextAwaitTaskContinuation">;
    use super::super::super::*;
    impl From<SynchronizationContextAwaitTaskContinuation> for System::Threading::Tasks::AwaitTaskContinuation {
     fn from(v:SynchronizationContextAwaitTaskContinuation)->System::Threading::Tasks::AwaitTaskContinuation{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Threading::Tasks::AwaitTaskContinuation,SynchronizationContextAwaitTaskContinuation>(v)
    }} 
    pub type TaskSchedulerAwaitTaskContinuation =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.Tasks.TaskSchedulerAwaitTaskContinuation">;
    use super::super::super::*;
    impl From<TaskSchedulerAwaitTaskContinuation> for System::Threading::Tasks::AwaitTaskContinuation {
     fn from(v:TaskSchedulerAwaitTaskContinuation)->System::Threading::Tasks::AwaitTaskContinuation{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Threading::Tasks::AwaitTaskContinuation,TaskSchedulerAwaitTaskContinuation>(v)
    }} 
    pub type AwaitTaskContinuation =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.Tasks.AwaitTaskContinuation">;
    use super::super::super::*;
    impl From<AwaitTaskContinuation> for System::Threading::Tasks::TaskContinuation {
     fn from(v:AwaitTaskContinuation)->System::Threading::Tasks::TaskContinuation{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Threading::Tasks::TaskContinuation,AwaitTaskContinuation>(v)
    }} 
    pub type TaskExceptionHolder =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.Tasks.TaskExceptionHolder">;
    use super::super::super::*;
    impl From<TaskExceptionHolder> for System::Object {
     fn from(v:TaskExceptionHolder)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,TaskExceptionHolder>(v)
    }} 
    pub type TaskExtensions =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.Tasks.TaskExtensions">;
    use super::super::super::*;
    impl From<TaskExtensions> for System::Object {
     fn from(v:TaskExtensions)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,TaskExtensions>(v)
    }} 
    pub type TaskFactory =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.Tasks.TaskFactory">;
    use super::super::super::*;
    impl From<TaskFactory> for System::Object {
     fn from(v:TaskFactory)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,TaskFactory>(v)
    }} 
    pub type TaskScheduler =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.Tasks.TaskScheduler">;
    use super::super::super::*;
    impl From<TaskScheduler> for System::Object {
     fn from(v:TaskScheduler)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,TaskScheduler>(v)
    }} 
    pub type SynchronizationContextTaskScheduler =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.Tasks.SynchronizationContextTaskScheduler">;
    use super::super::super::*;
    impl From<SynchronizationContextTaskScheduler> for System::Threading::Tasks::TaskScheduler {
     fn from(v:SynchronizationContextTaskScheduler)->System::Threading::Tasks::TaskScheduler{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Threading::Tasks::TaskScheduler,SynchronizationContextTaskScheduler>(v)
    }} 
    pub type UnobservedTaskExceptionEventArgs =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.Tasks.UnobservedTaskExceptionEventArgs">;
    use super::super::super::*;
    impl From<UnobservedTaskExceptionEventArgs> for System::EventArgs {
     fn from(v:UnobservedTaskExceptionEventArgs)->System::EventArgs{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::EventArgs,UnobservedTaskExceptionEventArgs>(v)
    }} 
    pub type TaskSchedulerException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.Tasks.TaskSchedulerException">;
    use super::super::super::*;
    impl From<TaskSchedulerException> for System::Exception {
     fn from(v:TaskSchedulerException)->System::Exception{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Exception,TaskSchedulerException>(v)
    }} 
    pub type ThreadPoolTaskScheduler =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.Tasks.ThreadPoolTaskScheduler">;
    use super::super::super::*;
    impl From<ThreadPoolTaskScheduler> for System::Threading::Tasks::TaskScheduler {
     fn from(v:ThreadPoolTaskScheduler)->System::Threading::Tasks::TaskScheduler{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Threading::Tasks::TaskScheduler,ThreadPoolTaskScheduler>(v)
    }} 
    pub type TplEventSource =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.Tasks.TplEventSource">;
    use super::super::super::*;
    impl From<TplEventSource> for System::Diagnostics::Tracing::EventSource {
     fn from(v:TplEventSource)->System::Diagnostics::Tracing::EventSource{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Diagnostics::Tracing::EventSource,TplEventSource>(v)
    }} 
    pub type TaskToAsyncResult =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.Tasks.TaskToAsyncResult">;
    use super::super::super::*;
    impl From<TaskToAsyncResult> for System::Object {
     fn from(v:TaskToAsyncResult)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,TaskToAsyncResult>(v)
    }} 
    }
    pub type Interlocked =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.Interlocked">;
    use super::super::*;
    impl From<Interlocked> for System::Object {
     fn from(v:Interlocked)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Interlocked>(v)
    }} 
    pub type Monitor =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.Monitor">;
    use super::super::*;
    impl From<Monitor> for System::Object {
     fn from(v:Monitor)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Monitor>(v)
    }} 
    pub type SynchronizationContext =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.SynchronizationContext">;
    use super::super::*;
    impl From<SynchronizationContext> for System::Object {
     fn from(v:SynchronizationContext)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,SynchronizationContext>(v)
    }} 
    pub type Thread =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.Thread">;
    use super::super::*;
    impl From<Thread> for System::Runtime::ConstrainedExecution::CriticalFinalizerObject {
     fn from(v:Thread)->System::Runtime::ConstrainedExecution::CriticalFinalizerObject{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Runtime::ConstrainedExecution::CriticalFinalizerObject,Thread>(v)
    }} 
    pub type ThreadPool =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.ThreadPool">;
    use super::super::*;
    impl From<ThreadPool> for System::Object {
     fn from(v:ThreadPool)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,ThreadPool>(v)
    }} 
    pub type WaitHandle =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.WaitHandle">;
    use super::super::*;
    impl From<WaitHandle> for System::MarshalByRefObject {
     fn from(v:WaitHandle)->System::MarshalByRefObject{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::MarshalByRefObject,WaitHandle>(v)
    }} 
    pub type LowLevelLifoSemaphore =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.LowLevelLifoSemaphore">;
    use super::super::*;
    impl From<LowLevelLifoSemaphore> for System::Threading::LowLevelLifoSemaphoreBase {
     fn from(v:LowLevelLifoSemaphore)->System::Threading::LowLevelLifoSemaphoreBase{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Threading::LowLevelLifoSemaphoreBase,LowLevelLifoSemaphore>(v)
    }} 
    pub type AbandonedMutexException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.AbandonedMutexException">;
    use super::super::*;
    impl From<AbandonedMutexException> for System::SystemException {
     fn from(v:AbandonedMutexException)->System::SystemException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::SystemException,AbandonedMutexException>(v)
    }} 
    pub type IAsyncLocal =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.IAsyncLocal">;
    use super::super::*;
    pub type IAsyncLocalValueMap =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.IAsyncLocalValueMap">;
    use super::super::*;
    pub type AsyncLocalValueMap =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.AsyncLocalValueMap">;
    use super::super::*;
    impl From<AsyncLocalValueMap> for System::Object {
     fn from(v:AsyncLocalValueMap)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,AsyncLocalValueMap>(v)
    }} 
    pub type AutoResetEvent =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.AutoResetEvent">;
    use super::super::*;
    impl From<AutoResetEvent> for System::Threading::EventWaitHandle {
     fn from(v:AutoResetEvent)->System::Threading::EventWaitHandle{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Threading::EventWaitHandle,AutoResetEvent>(v)
    }} 
    pub type CancellationTokenSource =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.CancellationTokenSource">;
    use super::super::*;
    impl From<CancellationTokenSource> for System::Object {
     fn from(v:CancellationTokenSource)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,CancellationTokenSource>(v)
    }} 
    pub type CompressedStack =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.CompressedStack">;
    use super::super::*;
    impl From<CompressedStack> for System::Object {
     fn from(v:CompressedStack)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,CompressedStack>(v)
    }} 
    pub type IDeferredDisposable =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.IDeferredDisposable">;
    use super::super::*;
    pub type EventWaitHandle =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.EventWaitHandle">;
    use super::super::*;
    impl From<EventWaitHandle> for System::Threading::WaitHandle {
     fn from(v:EventWaitHandle)->System::Threading::WaitHandle{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Threading::WaitHandle,EventWaitHandle>(v)
    }} 
    pub type ContextCallback =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.ContextCallback">;
    use super::super::*;
    impl From<ContextCallback> for System::MulticastDelegate {
     fn from(v:ContextCallback)->System::MulticastDelegate{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::MulticastDelegate,ContextCallback>(v)
    }} 
    pub type ExecutionContext =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.ExecutionContext">;
    use super::super::*;
    impl From<ExecutionContext> for System::Object {
     fn from(v:ExecutionContext)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,ExecutionContext>(v)
    }} 
    pub type IOCompletionCallback =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.IOCompletionCallback">;
    use super::super::*;
    impl From<IOCompletionCallback> for System::MulticastDelegate {
     fn from(v:IOCompletionCallback)->System::MulticastDelegate{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::MulticastDelegate,IOCompletionCallback>(v)
    }} 
    pub type IOCompletionCallbackHelper =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.IOCompletionCallbackHelper">;
    use super::super::*;
    impl From<IOCompletionCallbackHelper> for System::Object {
     fn from(v:IOCompletionCallbackHelper)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,IOCompletionCallbackHelper>(v)
    }} 
    pub type IThreadPoolWorkItem =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.IThreadPoolWorkItem">;
    use super::super::*;
    pub type LazyInitializer =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.LazyInitializer">;
    use super::super::*;
    impl From<LazyInitializer> for System::Object {
     fn from(v:LazyInitializer)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,LazyInitializer>(v)
    }} 
    pub type LockRecursionException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.LockRecursionException">;
    use super::super::*;
    impl From<LockRecursionException> for System::Exception {
     fn from(v:LockRecursionException)->System::Exception{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Exception,LockRecursionException>(v)
    }} 
    pub type LowLevelLock =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.LowLevelLock">;
    use super::super::*;
    impl From<LowLevelLock> for System::Object {
     fn from(v:LowLevelLock)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,LowLevelLock>(v)
    }} 
    pub type ManualResetEvent =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.ManualResetEvent">;
    use super::super::*;
    impl From<ManualResetEvent> for System::Threading::EventWaitHandle {
     fn from(v:ManualResetEvent)->System::Threading::EventWaitHandle{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Threading::EventWaitHandle,ManualResetEvent>(v)
    }} 
    pub type ManualResetEventSlim =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.ManualResetEventSlim">;
    use super::super::*;
    impl From<ManualResetEventSlim> for System::Object {
     fn from(v:ManualResetEventSlim)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,ManualResetEventSlim>(v)
    }} 
    pub type Mutex =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.Mutex">;
    use super::super::*;
    impl From<Mutex> for System::Threading::WaitHandle {
     fn from(v:Mutex)->System::Threading::WaitHandle{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Threading::WaitHandle,Mutex>(v)
    }} 
    pub type Overlapped =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.Overlapped">;
    use super::super::*;
    impl From<Overlapped> for System::Object {
     fn from(v:Overlapped)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Overlapped>(v)
    }} 
    pub type ParameterizedThreadStart =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.ParameterizedThreadStart">;
    use super::super::*;
    impl From<ParameterizedThreadStart> for System::MulticastDelegate {
     fn from(v:ParameterizedThreadStart)->System::MulticastDelegate{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::MulticastDelegate,ParameterizedThreadStart>(v)
    }} 
    pub type ReaderWriterCount =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.ReaderWriterCount">;
    use super::super::*;
    impl From<ReaderWriterCount> for System::Object {
     fn from(v:ReaderWriterCount)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,ReaderWriterCount>(v)
    }} 
    pub type ReaderWriterLockSlim =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.ReaderWriterLockSlim">;
    use super::super::*;
    impl From<ReaderWriterLockSlim> for System::Object {
     fn from(v:ReaderWriterLockSlim)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,ReaderWriterLockSlim>(v)
    }} 
    pub type Semaphore =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.Semaphore">;
    use super::super::*;
    impl From<Semaphore> for System::Threading::WaitHandle {
     fn from(v:Semaphore)->System::Threading::WaitHandle{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Threading::WaitHandle,Semaphore>(v)
    }} 
    pub type SemaphoreFullException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.SemaphoreFullException">;
    use super::super::*;
    impl From<SemaphoreFullException> for System::SystemException {
     fn from(v:SemaphoreFullException)->System::SystemException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::SystemException,SemaphoreFullException>(v)
    }} 
    pub type SemaphoreSlim =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.SemaphoreSlim">;
    use super::super::*;
    impl From<SemaphoreSlim> for System::Object {
     fn from(v:SemaphoreSlim)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,SemaphoreSlim>(v)
    }} 
    pub type SendOrPostCallback =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.SendOrPostCallback">;
    use super::super::*;
    impl From<SendOrPostCallback> for System::MulticastDelegate {
     fn from(v:SendOrPostCallback)->System::MulticastDelegate{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::MulticastDelegate,SendOrPostCallback>(v)
    }} 
    pub type SynchronizationLockException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.SynchronizationLockException">;
    use super::super::*;
    impl From<SynchronizationLockException> for System::SystemException {
     fn from(v:SynchronizationLockException)->System::SystemException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::SystemException,SynchronizationLockException>(v)
    }} 
    pub type ProcessorIdCache =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.ProcessorIdCache">;
    use super::super::*;
    impl From<ProcessorIdCache> for System::Object {
     fn from(v:ProcessorIdCache)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,ProcessorIdCache>(v)
    }} 
    pub type ThreadAbortException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.ThreadAbortException">;
    use super::super::*;
    impl From<ThreadAbortException> for System::SystemException {
     fn from(v:ThreadAbortException)->System::SystemException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::SystemException,ThreadAbortException>(v)
    }} 
    pub type ThreadExceptionEventArgs =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.ThreadExceptionEventArgs">;
    use super::super::*;
    impl From<ThreadExceptionEventArgs> for System::EventArgs {
     fn from(v:ThreadExceptionEventArgs)->System::EventArgs{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::EventArgs,ThreadExceptionEventArgs>(v)
    }} 
    pub type ThreadExceptionEventHandler =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.ThreadExceptionEventHandler">;
    use super::super::*;
    impl From<ThreadExceptionEventHandler> for System::MulticastDelegate {
     fn from(v:ThreadExceptionEventHandler)->System::MulticastDelegate{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::MulticastDelegate,ThreadExceptionEventHandler>(v)
    }} 
    pub type ThreadInt64PersistentCounter =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.ThreadInt64PersistentCounter">;
    use super::super::*;
    impl From<ThreadInt64PersistentCounter> for System::Object {
     fn from(v:ThreadInt64PersistentCounter)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,ThreadInt64PersistentCounter>(v)
    }} 
    pub type ThreadInterruptedException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.ThreadInterruptedException">;
    use super::super::*;
    impl From<ThreadInterruptedException> for System::SystemException {
     fn from(v:ThreadInterruptedException)->System::SystemException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::SystemException,ThreadInterruptedException>(v)
    }} 
    pub type ThreadPoolWorkQueue =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.ThreadPoolWorkQueue">;
    use super::super::*;
    impl From<ThreadPoolWorkQueue> for System::Object {
     fn from(v:ThreadPoolWorkQueue)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,ThreadPoolWorkQueue>(v)
    }} 
    pub type ThreadPoolWorkQueueThreadLocals =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.ThreadPoolWorkQueueThreadLocals">;
    use super::super::*;
    impl From<ThreadPoolWorkQueueThreadLocals> for System::Object {
     fn from(v:ThreadPoolWorkQueueThreadLocals)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,ThreadPoolWorkQueueThreadLocals>(v)
    }} 
    pub type WaitCallback =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.WaitCallback">;
    use super::super::*;
    impl From<WaitCallback> for System::MulticastDelegate {
     fn from(v:WaitCallback)->System::MulticastDelegate{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::MulticastDelegate,WaitCallback>(v)
    }} 
    pub type WaitOrTimerCallback =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.WaitOrTimerCallback">;
    use super::super::*;
    impl From<WaitOrTimerCallback> for System::MulticastDelegate {
     fn from(v:WaitOrTimerCallback)->System::MulticastDelegate{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::MulticastDelegate,WaitOrTimerCallback>(v)
    }} 
    pub type QueueUserWorkItemCallbackBase =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.QueueUserWorkItemCallbackBase">;
    use super::super::*;
    impl From<QueueUserWorkItemCallbackBase> for System::Object {
     fn from(v:QueueUserWorkItemCallbackBase)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,QueueUserWorkItemCallbackBase>(v)
    }} 
    pub type QueueUserWorkItemCallback =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.QueueUserWorkItemCallback">;
    use super::super::*;
    impl From<QueueUserWorkItemCallback> for System::Threading::QueueUserWorkItemCallbackBase {
     fn from(v:QueueUserWorkItemCallback)->System::Threading::QueueUserWorkItemCallbackBase{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Threading::QueueUserWorkItemCallbackBase,QueueUserWorkItemCallback>(v)
    }} 
    pub type QueueUserWorkItemCallbackDefaultContext =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.QueueUserWorkItemCallbackDefaultContext">;
    use super::super::*;
    impl From<QueueUserWorkItemCallbackDefaultContext> for System::Threading::QueueUserWorkItemCallbackBase {
     fn from(v:QueueUserWorkItemCallbackDefaultContext)->System::Threading::QueueUserWorkItemCallbackBase{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Threading::QueueUserWorkItemCallbackBase,QueueUserWorkItemCallbackDefaultContext>(v)
    }} 
    pub type _ThreadPoolWaitOrTimerCallback =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading._ThreadPoolWaitOrTimerCallback">;
    use super::super::*;
    impl From<_ThreadPoolWaitOrTimerCallback> for System::Object {
     fn from(v:_ThreadPoolWaitOrTimerCallback)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,_ThreadPoolWaitOrTimerCallback>(v)
    }} 
    pub type ThreadStart =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.ThreadStart">;
    use super::super::*;
    impl From<ThreadStart> for System::MulticastDelegate {
     fn from(v:ThreadStart)->System::MulticastDelegate{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::MulticastDelegate,ThreadStart>(v)
    }} 
    pub type ThreadStartException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.ThreadStartException">;
    use super::super::*;
    impl From<ThreadStartException> for System::SystemException {
     fn from(v:ThreadStartException)->System::SystemException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::SystemException,ThreadStartException>(v)
    }} 
    pub type ThreadStateException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.ThreadStateException">;
    use super::super::*;
    impl From<ThreadStateException> for System::SystemException {
     fn from(v:ThreadStateException)->System::SystemException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::SystemException,ThreadStateException>(v)
    }} 
    pub type Timeout =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.Timeout">;
    use super::super::*;
    impl From<Timeout> for System::Object {
     fn from(v:Timeout)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Timeout>(v)
    }} 
    pub type TimeoutHelper =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.TimeoutHelper">;
    use super::super::*;
    impl From<TimeoutHelper> for System::Object {
     fn from(v:TimeoutHelper)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,TimeoutHelper>(v)
    }} 
    pub type PeriodicTimer =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.PeriodicTimer">;
    use super::super::*;
    impl From<PeriodicTimer> for System::Object {
     fn from(v:PeriodicTimer)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,PeriodicTimer>(v)
    }} 
    pub type TimerCallback =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.TimerCallback">;
    use super::super::*;
    impl From<TimerCallback> for System::MulticastDelegate {
     fn from(v:TimerCallback)->System::MulticastDelegate{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::MulticastDelegate,TimerCallback>(v)
    }} 
    pub type TimerQueue =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.TimerQueue">;
    use super::super::*;
    impl From<TimerQueue> for System::Object {
     fn from(v:TimerQueue)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,TimerQueue>(v)
    }} 
    pub type TimerQueueTimer =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.TimerQueueTimer">;
    use super::super::*;
    impl From<TimerQueueTimer> for System::Object {
     fn from(v:TimerQueueTimer)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,TimerQueueTimer>(v)
    }} 
    pub type TimerHolder =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.TimerHolder">;
    use super::super::*;
    impl From<TimerHolder> for System::Object {
     fn from(v:TimerHolder)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,TimerHolder>(v)
    }} 
    pub type Timer =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.Timer">;
    use super::super::*;
    impl From<Timer> for System::MarshalByRefObject {
     fn from(v:Timer)->System::MarshalByRefObject{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::MarshalByRefObject,Timer>(v)
    }} 
    pub type Volatile =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.Volatile">;
    use super::super::*;
    impl From<Volatile> for System::Object {
     fn from(v:Volatile)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Volatile>(v)
    }} 
    pub type WaitHandleCannotBeOpenedException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.WaitHandleCannotBeOpenedException">;
    use super::super::*;
    impl From<WaitHandleCannotBeOpenedException> for System::ApplicationException {
     fn from(v:WaitHandleCannotBeOpenedException)->System::ApplicationException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::ApplicationException,WaitHandleCannotBeOpenedException>(v)
    }} 
    pub type WaitHandleExtensions =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.WaitHandleExtensions">;
    use super::super::*;
    impl From<WaitHandleExtensions> for System::Object {
     fn from(v:WaitHandleExtensions)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,WaitHandleExtensions>(v)
    }} 
    pub type ITimer =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.ITimer">;
    use super::super::*;
    pub type CompleteWaitThreadPoolWorkItem =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.CompleteWaitThreadPoolWorkItem">;
    use super::super::*;
    impl From<CompleteWaitThreadPoolWorkItem> for System::Object {
     fn from(v:CompleteWaitThreadPoolWorkItem)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,CompleteWaitThreadPoolWorkItem>(v)
    }} 
    pub type PortableThreadPool =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.PortableThreadPool">;
    use super::super::*;
    impl From<PortableThreadPool> for System::Object {
     fn from(v:PortableThreadPool)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,PortableThreadPool>(v)
    }} 
    pub type PreAllocatedOverlapped =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.PreAllocatedOverlapped">;
    use super::super::*;
    impl From<PreAllocatedOverlapped> for System::Object {
     fn from(v:PreAllocatedOverlapped)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,PreAllocatedOverlapped>(v)
    }} 
    pub type RegisteredWaitHandle =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.RegisteredWaitHandle">;
    use super::super::*;
    impl From<RegisteredWaitHandle> for System::MarshalByRefObject {
     fn from(v:RegisteredWaitHandle)->System::MarshalByRefObject{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::MarshalByRefObject,RegisteredWaitHandle>(v)
    }} 
    pub type LowLevelLifoSemaphoreBase =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.LowLevelLifoSemaphoreBase">;
    use super::super::*;
    impl From<LowLevelLifoSemaphoreBase> for System::Object {
     fn from(v:LowLevelLifoSemaphoreBase)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,LowLevelLifoSemaphoreBase>(v)
    }} 
    pub type ThreadPoolBoundHandle =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.ThreadPoolBoundHandle">;
    use super::super::*;
    impl From<ThreadPoolBoundHandle> for System::Object {
     fn from(v:ThreadPoolBoundHandle)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,ThreadPoolBoundHandle>(v)
    }} 
    pub type ThreadPoolBoundHandleOverlapped =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Threading.ThreadPoolBoundHandleOverlapped">;
    use super::super::*;
    impl From<ThreadPoolBoundHandleOverlapped> for System::Threading::Overlapped {
     fn from(v:ThreadPoolBoundHandleOverlapped)->System::Threading::Overlapped{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Threading::Overlapped,ThreadPoolBoundHandleOverlapped>(v)
    }} 
    }
    pub mod Buffers{
    pub mod Binary{
    pub type BinaryPrimitives =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Buffers.Binary.BinaryPrimitives">;
    use super::super::super::*;
    impl From<BinaryPrimitives> for System::Object {
     fn from(v:BinaryPrimitives)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,BinaryPrimitives>(v)
    }} 
    }
    pub mod Text{
    pub type Base64 =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Buffers.Text.Base64">;
    use super::super::super::*;
    impl From<Base64> for System::Object {
     fn from(v:Base64)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Base64>(v)
    }} 
    pub type FormattingHelpers =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Buffers.Text.FormattingHelpers">;
    use super::super::super::*;
    impl From<FormattingHelpers> for System::Object {
     fn from(v:FormattingHelpers)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,FormattingHelpers>(v)
    }} 
    pub type Utf8Formatter =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Buffers.Text.Utf8Formatter">;
    use super::super::super::*;
    impl From<Utf8Formatter> for System::Object {
     fn from(v:Utf8Formatter)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Utf8Formatter>(v)
    }} 
    pub type ParserHelpers =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Buffers.Text.ParserHelpers">;
    use super::super::super::*;
    impl From<ParserHelpers> for System::Object {
     fn from(v:ParserHelpers)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,ParserHelpers>(v)
    }} 
    pub type Utf8Parser =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Buffers.Text.Utf8Parser">;
    use super::super::super::*;
    impl From<Utf8Parser> for System::Object {
     fn from(v:Utf8Parser)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Utf8Parser>(v)
    }} 
    }
    pub type ArrayPoolEventSource =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Buffers.ArrayPoolEventSource">;
    use super::super::*;
    impl From<ArrayPoolEventSource> for System::Diagnostics::Tracing::EventSource {
     fn from(v:ArrayPoolEventSource)->System::Diagnostics::Tracing::EventSource{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Diagnostics::Tracing::EventSource,ArrayPoolEventSource>(v)
    }} 
    pub type IPinnable =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Buffers.IPinnable">;
    use super::super::*;
    pub type SharedArrayPoolStatics =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Buffers.SharedArrayPoolStatics">;
    use super::super::*;
    impl From<SharedArrayPoolStatics> for System::Object {
     fn from(v:SharedArrayPoolStatics)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,SharedArrayPoolStatics>(v)
    }} 
    pub type Utilities =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Buffers.Utilities">;
    use super::super::*;
    impl From<Utilities> for System::Object {
     fn from(v:Utilities)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Utilities>(v)
    }} 
    pub type SingleByteSearchValues =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Buffers.SingleByteSearchValues">;
    use super::super::*;
    pub type Any2ByteSearchValues =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Buffers.Any2ByteSearchValues">;
    use super::super::*;
    pub type Any3ByteSearchValues =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Buffers.Any3ByteSearchValues">;
    use super::super::*;
    pub type AsciiByteSearchValues =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Buffers.AsciiByteSearchValues">;
    use super::super::*;
    pub type IndexOfAnyAsciiSearcher =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Buffers.IndexOfAnyAsciiSearcher">;
    use super::super::*;
    impl From<IndexOfAnyAsciiSearcher> for System::Object {
     fn from(v:IndexOfAnyAsciiSearcher)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,IndexOfAnyAsciiSearcher>(v)
    }} 
    pub type AnyByteSearchValues =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Buffers.AnyByteSearchValues">;
    use super::super::*;
    pub type RangeByteSearchValues =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Buffers.RangeByteSearchValues">;
    use super::super::*;
    pub type ProbabilisticCharSearchValues =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Buffers.ProbabilisticCharSearchValues">;
    use super::super::*;
    pub type Latin1CharSearchValues =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Buffers.Latin1CharSearchValues">;
    use super::super::*;
    pub type SearchValues =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Buffers.SearchValues">;
    use super::super::*;
    impl From<SearchValues> for System::Object {
     fn from(v:SearchValues)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,SearchValues>(v)
    }} 
    }
    pub mod Text{
    pub mod Unicode{
    pub type TextSegmentationUtility =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Text.Unicode.TextSegmentationUtility">;
    use super::super::super::*;
    impl From<TextSegmentationUtility> for System::Object {
     fn from(v:TextSegmentationUtility)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,TextSegmentationUtility>(v)
    }} 
    pub type Utf16Utility =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Text.Unicode.Utf16Utility">;
    use super::super::super::*;
    impl From<Utf16Utility> for System::Object {
     fn from(v:Utf16Utility)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Utf16Utility>(v)
    }} 
    pub type Utf8 =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Text.Unicode.Utf8">;
    use super::super::super::*;
    impl From<Utf8> for System::Object {
     fn from(v:Utf8)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Utf8>(v)
    }} 
    pub type Utf8Utility =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Text.Unicode.Utf8Utility">;
    use super::super::super::*;
    impl From<Utf8Utility> for System::Object {
     fn from(v:Utf8Utility)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Utf8Utility>(v)
    }} 
    }
    pub type StringBuilder =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Text.StringBuilder">;
    use super::super::*;
    impl From<StringBuilder> for System::Object {
     fn from(v:StringBuilder)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,StringBuilder>(v)
    }} 
    pub type Ascii =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Text.Ascii">;
    use super::super::*;
    impl From<Ascii> for System::Object {
     fn from(v:Ascii)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Ascii>(v)
    }} 
    pub type ASCIIEncoding =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Text.ASCIIEncoding">;
    use super::super::*;
    impl From<ASCIIEncoding> for System::Text::Encoding {
     fn from(v:ASCIIEncoding)->System::Text::Encoding{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Text::Encoding,ASCIIEncoding>(v)
    }} 
    pub type CodePageDataItem =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Text.CodePageDataItem">;
    use super::super::*;
    impl From<CodePageDataItem> for System::Object {
     fn from(v:CodePageDataItem)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,CodePageDataItem>(v)
    }} 
    pub type CompositeFormat =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Text.CompositeFormat">;
    use super::super::*;
    impl From<CompositeFormat> for System::Object {
     fn from(v:CompositeFormat)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,CompositeFormat>(v)
    }} 
    pub type Decoder =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Text.Decoder">;
    use super::super::*;
    impl From<Decoder> for System::Object {
     fn from(v:Decoder)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Decoder>(v)
    }} 
    pub type DecoderExceptionFallback =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Text.DecoderExceptionFallback">;
    use super::super::*;
    impl From<DecoderExceptionFallback> for System::Text::DecoderFallback {
     fn from(v:DecoderExceptionFallback)->System::Text::DecoderFallback{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Text::DecoderFallback,DecoderExceptionFallback>(v)
    }} 
    pub type DecoderExceptionFallbackBuffer =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Text.DecoderExceptionFallbackBuffer">;
    use super::super::*;
    impl From<DecoderExceptionFallbackBuffer> for System::Text::DecoderFallbackBuffer {
     fn from(v:DecoderExceptionFallbackBuffer)->System::Text::DecoderFallbackBuffer{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Text::DecoderFallbackBuffer,DecoderExceptionFallbackBuffer>(v)
    }} 
    pub type DecoderFallbackException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Text.DecoderFallbackException">;
    use super::super::*;
    impl From<DecoderFallbackException> for System::ArgumentException {
     fn from(v:DecoderFallbackException)->System::ArgumentException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::ArgumentException,DecoderFallbackException>(v)
    }} 
    pub type DecoderFallback =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Text.DecoderFallback">;
    use super::super::*;
    impl From<DecoderFallback> for System::Object {
     fn from(v:DecoderFallback)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,DecoderFallback>(v)
    }} 
    pub type DecoderFallbackBuffer =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Text.DecoderFallbackBuffer">;
    use super::super::*;
    impl From<DecoderFallbackBuffer> for System::Object {
     fn from(v:DecoderFallbackBuffer)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,DecoderFallbackBuffer>(v)
    }} 
    pub type DecoderNLS =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Text.DecoderNLS">;
    use super::super::*;
    impl From<DecoderNLS> for System::Text::Decoder {
     fn from(v:DecoderNLS)->System::Text::Decoder{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Text::Decoder,DecoderNLS>(v)
    }} 
    pub type DecoderReplacementFallback =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Text.DecoderReplacementFallback">;
    use super::super::*;
    impl From<DecoderReplacementFallback> for System::Text::DecoderFallback {
     fn from(v:DecoderReplacementFallback)->System::Text::DecoderFallback{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Text::DecoderFallback,DecoderReplacementFallback>(v)
    }} 
    pub type DecoderReplacementFallbackBuffer =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Text.DecoderReplacementFallbackBuffer">;
    use super::super::*;
    impl From<DecoderReplacementFallbackBuffer> for System::Text::DecoderFallbackBuffer {
     fn from(v:DecoderReplacementFallbackBuffer)->System::Text::DecoderFallbackBuffer{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Text::DecoderFallbackBuffer,DecoderReplacementFallbackBuffer>(v)
    }} 
    pub type Encoder =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Text.Encoder">;
    use super::super::*;
    impl From<Encoder> for System::Object {
     fn from(v:Encoder)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Encoder>(v)
    }} 
    pub type EncoderExceptionFallback =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Text.EncoderExceptionFallback">;
    use super::super::*;
    impl From<EncoderExceptionFallback> for System::Text::EncoderFallback {
     fn from(v:EncoderExceptionFallback)->System::Text::EncoderFallback{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Text::EncoderFallback,EncoderExceptionFallback>(v)
    }} 
    pub type EncoderExceptionFallbackBuffer =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Text.EncoderExceptionFallbackBuffer">;
    use super::super::*;
    impl From<EncoderExceptionFallbackBuffer> for System::Text::EncoderFallbackBuffer {
     fn from(v:EncoderExceptionFallbackBuffer)->System::Text::EncoderFallbackBuffer{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Text::EncoderFallbackBuffer,EncoderExceptionFallbackBuffer>(v)
    }} 
    pub type EncoderFallbackException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Text.EncoderFallbackException">;
    use super::super::*;
    impl From<EncoderFallbackException> for System::ArgumentException {
     fn from(v:EncoderFallbackException)->System::ArgumentException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::ArgumentException,EncoderFallbackException>(v)
    }} 
    pub type EncoderFallback =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Text.EncoderFallback">;
    use super::super::*;
    impl From<EncoderFallback> for System::Object {
     fn from(v:EncoderFallback)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,EncoderFallback>(v)
    }} 
    pub type EncoderFallbackBuffer =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Text.EncoderFallbackBuffer">;
    use super::super::*;
    impl From<EncoderFallbackBuffer> for System::Object {
     fn from(v:EncoderFallbackBuffer)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,EncoderFallbackBuffer>(v)
    }} 
    pub type EncoderLatin1BestFitFallback =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Text.EncoderLatin1BestFitFallback">;
    use super::super::*;
    impl From<EncoderLatin1BestFitFallback> for System::Text::EncoderFallback {
     fn from(v:EncoderLatin1BestFitFallback)->System::Text::EncoderFallback{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Text::EncoderFallback,EncoderLatin1BestFitFallback>(v)
    }} 
    pub type EncoderLatin1BestFitFallbackBuffer =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Text.EncoderLatin1BestFitFallbackBuffer">;
    use super::super::*;
    impl From<EncoderLatin1BestFitFallbackBuffer> for System::Text::EncoderFallbackBuffer {
     fn from(v:EncoderLatin1BestFitFallbackBuffer)->System::Text::EncoderFallbackBuffer{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Text::EncoderFallbackBuffer,EncoderLatin1BestFitFallbackBuffer>(v)
    }} 
    pub type EncoderNLS =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Text.EncoderNLS">;
    use super::super::*;
    impl From<EncoderNLS> for System::Text::Encoder {
     fn from(v:EncoderNLS)->System::Text::Encoder{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Text::Encoder,EncoderNLS>(v)
    }} 
    pub type EncoderReplacementFallback =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Text.EncoderReplacementFallback">;
    use super::super::*;
    impl From<EncoderReplacementFallback> for System::Text::EncoderFallback {
     fn from(v:EncoderReplacementFallback)->System::Text::EncoderFallback{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Text::EncoderFallback,EncoderReplacementFallback>(v)
    }} 
    pub type EncoderReplacementFallbackBuffer =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Text.EncoderReplacementFallbackBuffer">;
    use super::super::*;
    impl From<EncoderReplacementFallbackBuffer> for System::Text::EncoderFallbackBuffer {
     fn from(v:EncoderReplacementFallbackBuffer)->System::Text::EncoderFallbackBuffer{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Text::EncoderFallbackBuffer,EncoderReplacementFallbackBuffer>(v)
    }} 
    pub type Encoding =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Text.Encoding">;
    use super::super::*;
    impl From<Encoding> for System::Object {
     fn from(v:Encoding)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Encoding>(v)
    }} 
    pub type EncodingTable =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Text.EncodingTable">;
    use super::super::*;
    impl From<EncodingTable> for System::Object {
     fn from(v:EncodingTable)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,EncodingTable>(v)
    }} 
    pub type EncodingInfo =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Text.EncodingInfo">;
    use super::super::*;
    impl From<EncodingInfo> for System::Object {
     fn from(v:EncodingInfo)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,EncodingInfo>(v)
    }} 
    pub type EncodingProvider =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Text.EncodingProvider">;
    use super::super::*;
    impl From<EncodingProvider> for System::Object {
     fn from(v:EncodingProvider)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,EncodingProvider>(v)
    }} 
    pub type Latin1Encoding =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Text.Latin1Encoding">;
    use super::super::*;
    impl From<Latin1Encoding> for System::Text::Encoding {
     fn from(v:Latin1Encoding)->System::Text::Encoding{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Text::Encoding,Latin1Encoding>(v)
    }} 
    pub type Latin1Utility =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Text.Latin1Utility">;
    use super::super::*;
    impl From<Latin1Utility> for System::Object {
     fn from(v:Latin1Utility)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Latin1Utility>(v)
    }} 
    pub type TranscodingStream =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Text.TranscodingStream">;
    use super::super::*;
    impl From<TranscodingStream> for System::IO::Stream {
     fn from(v:TranscodingStream)->System::IO::Stream{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::IO::Stream,TranscodingStream>(v)
    }} 
    pub type UnicodeEncoding =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Text.UnicodeEncoding">;
    use super::super::*;
    impl From<UnicodeEncoding> for System::Text::Encoding {
     fn from(v:UnicodeEncoding)->System::Text::Encoding{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Text::Encoding,UnicodeEncoding>(v)
    }} 
    pub type UnicodeUtility =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Text.UnicodeUtility">;
    use super::super::*;
    impl From<UnicodeUtility> for System::Object {
     fn from(v:UnicodeUtility)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,UnicodeUtility>(v)
    }} 
    pub type UTF32Encoding =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Text.UTF32Encoding">;
    use super::super::*;
    impl From<UTF32Encoding> for System::Text::Encoding {
     fn from(v:UTF32Encoding)->System::Text::Encoding{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Text::Encoding,UTF32Encoding>(v)
    }} 
    pub type UTF7Encoding =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Text.UTF7Encoding">;
    use super::super::*;
    impl From<UTF7Encoding> for System::Text::Encoding {
     fn from(v:UTF7Encoding)->System::Text::Encoding{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Text::Encoding,UTF7Encoding>(v)
    }} 
    pub type UTF8Encoding =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Text.UTF8Encoding">;
    use super::super::*;
    impl From<UTF8Encoding> for System::Text::Encoding {
     fn from(v:UTF8Encoding)->System::Text::Encoding{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Text::Encoding,UTF8Encoding>(v)
    }} 
    pub type StringBuilderCache =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Text.StringBuilderCache">;
    use super::super::*;
    impl From<StringBuilderCache> for System::Object {
     fn from(v:StringBuilderCache)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,StringBuilderCache>(v)
    }} 
    }
    pub mod Private{
    pub mod CoreLib{
    pub type Strings =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Private.CoreLib.Strings">;
    use super::super::super::*;
    impl From<Strings> for System::Object {
     fn from(v:Strings)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Strings>(v)
    }} 
    }
    }
    pub mod Net{
    pub type WebUtility =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Net.WebUtility">;
    use super::super::*;
    impl From<WebUtility> for System::Object {
     fn from(v:WebUtility)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,WebUtility>(v)
    }} 
    }
    pub mod Configuration{
    pub mod Assemblies{
    }
    }
    pub mod CodeDom{
    pub mod Compiler{
    pub type GeneratedCodeAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.CodeDom.Compiler.GeneratedCodeAttribute">;
    use super::super::super::*;
    impl From<GeneratedCodeAttribute> for System::Attribute {
     fn from(v:GeneratedCodeAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,GeneratedCodeAttribute>(v)
    }} 
    pub type IndentedTextWriter =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.CodeDom.Compiler.IndentedTextWriter">;
    use super::super::super::*;
    impl From<IndentedTextWriter> for System::IO::TextWriter {
     fn from(v:IndentedTextWriter)->System::IO::TextWriter{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::IO::TextWriter,IndentedTextWriter>(v)
    }} 
    }
    }
    pub mod Reflection{
    pub mod Emit{
    pub type CustomAttributeBuilder =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.Emit.CustomAttributeBuilder">;
    use super::super::super::*;
    impl From<CustomAttributeBuilder> for System::Object {
     fn from(v:CustomAttributeBuilder)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,CustomAttributeBuilder>(v)
    }} 
    pub type DynamicILGenerator =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.Emit.DynamicILGenerator">;
    use super::super::super::*;
    impl From<DynamicILGenerator> for System::Reflection::Emit::RuntimeILGenerator {
     fn from(v:DynamicILGenerator)->System::Reflection::Emit::RuntimeILGenerator{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::Emit::RuntimeILGenerator,DynamicILGenerator>(v)
    }} 
    pub type DynamicResolver =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.Emit.DynamicResolver">;
    use super::super::super::*;
    impl From<DynamicResolver> for System::Resolver {
     fn from(v:DynamicResolver)->System::Resolver{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Resolver,DynamicResolver>(v)
    }} 
    pub type DynamicILInfo =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.Emit.DynamicILInfo">;
    use super::super::super::*;
    impl From<DynamicILInfo> for System::Object {
     fn from(v:DynamicILInfo)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,DynamicILInfo>(v)
    }} 
    pub type DynamicScope =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.Emit.DynamicScope">;
    use super::super::super::*;
    impl From<DynamicScope> for System::Object {
     fn from(v:DynamicScope)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,DynamicScope>(v)
    }} 
    pub type GenericMethodInfo =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.Emit.GenericMethodInfo">;
    use super::super::super::*;
    impl From<GenericMethodInfo> for System::Object {
     fn from(v:GenericMethodInfo)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,GenericMethodInfo>(v)
    }} 
    pub type GenericFieldInfo =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.Emit.GenericFieldInfo">;
    use super::super::super::*;
    impl From<GenericFieldInfo> for System::Object {
     fn from(v:GenericFieldInfo)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,GenericFieldInfo>(v)
    }} 
    pub type VarArgMethod =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.Emit.VarArgMethod">;
    use super::super::super::*;
    impl From<VarArgMethod> for System::Object {
     fn from(v:VarArgMethod)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,VarArgMethod>(v)
    }} 
    pub type DynamicMethod =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.Emit.DynamicMethod">;
    use super::super::super::*;
    impl From<DynamicMethod> for System::Reflection::MethodInfo {
     fn from(v:DynamicMethod)->System::Reflection::MethodInfo{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::MethodInfo,DynamicMethod>(v)
    }} 
    pub type LocalBuilder =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.Emit.LocalBuilder">;
    use super::super::super::*;
    impl From<LocalBuilder> for System::Reflection::LocalVariableInfo {
     fn from(v:LocalBuilder)->System::Reflection::LocalVariableInfo{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::LocalVariableInfo,LocalBuilder>(v)
    }} 
    pub type AssemblyBuilder =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.Emit.AssemblyBuilder">;
    use super::super::super::*;
    impl From<AssemblyBuilder> for System::Reflection::Assembly {
     fn from(v:AssemblyBuilder)->System::Reflection::Assembly{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::Assembly,AssemblyBuilder>(v)
    }} 
    pub type RuntimeAssemblyBuilder =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.Emit.RuntimeAssemblyBuilder">;
    use super::super::super::*;
    impl From<RuntimeAssemblyBuilder> for System::Reflection::Emit::AssemblyBuilder {
     fn from(v:RuntimeAssemblyBuilder)->System::Reflection::Emit::AssemblyBuilder{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::Emit::AssemblyBuilder,RuntimeAssemblyBuilder>(v)
    }} 
    pub type RuntimeConstructorBuilder =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.Emit.RuntimeConstructorBuilder">;
    use super::super::super::*;
    impl From<RuntimeConstructorBuilder> for System::Reflection::Emit::ConstructorBuilder {
     fn from(v:RuntimeConstructorBuilder)->System::Reflection::Emit::ConstructorBuilder{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::Emit::ConstructorBuilder,RuntimeConstructorBuilder>(v)
    }} 
    pub type RuntimeEnumBuilder =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.Emit.RuntimeEnumBuilder">;
    use super::super::super::*;
    impl From<RuntimeEnumBuilder> for System::Reflection::Emit::EnumBuilder {
     fn from(v:RuntimeEnumBuilder)->System::Reflection::Emit::EnumBuilder{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::Emit::EnumBuilder,RuntimeEnumBuilder>(v)
    }} 
    pub type RuntimeEventBuilder =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.Emit.RuntimeEventBuilder">;
    use super::super::super::*;
    impl From<RuntimeEventBuilder> for System::Reflection::Emit::EventBuilder {
     fn from(v:RuntimeEventBuilder)->System::Reflection::Emit::EventBuilder{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::Emit::EventBuilder,RuntimeEventBuilder>(v)
    }} 
    pub type RuntimeFieldBuilder =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.Emit.RuntimeFieldBuilder">;
    use super::super::super::*;
    impl From<RuntimeFieldBuilder> for System::Reflection::Emit::FieldBuilder {
     fn from(v:RuntimeFieldBuilder)->System::Reflection::Emit::FieldBuilder{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::Emit::FieldBuilder,RuntimeFieldBuilder>(v)
    }} 
    pub type RuntimeGenericTypeParameterBuilder =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.Emit.RuntimeGenericTypeParameterBuilder">;
    use super::super::super::*;
    impl From<RuntimeGenericTypeParameterBuilder> for System::Reflection::Emit::GenericTypeParameterBuilder {
     fn from(v:RuntimeGenericTypeParameterBuilder)->System::Reflection::Emit::GenericTypeParameterBuilder{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::Emit::GenericTypeParameterBuilder,RuntimeGenericTypeParameterBuilder>(v)
    }} 
    pub type RuntimeILGenerator =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.Emit.RuntimeILGenerator">;
    use super::super::super::*;
    impl From<RuntimeILGenerator> for System::Reflection::Emit::ILGenerator {
     fn from(v:RuntimeILGenerator)->System::Reflection::Emit::ILGenerator{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::Emit::ILGenerator,RuntimeILGenerator>(v)
    }} 
    pub type __ExceptionInfo =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.Emit.__ExceptionInfo">;
    use super::super::super::*;
    impl From<__ExceptionInfo> for System::Object {
     fn from(v:__ExceptionInfo)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,__ExceptionInfo>(v)
    }} 
    pub type ScopeTree =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.Emit.ScopeTree">;
    use super::super::super::*;
    impl From<ScopeTree> for System::Object {
     fn from(v:ScopeTree)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,ScopeTree>(v)
    }} 
    pub type RuntimeMethodBuilder =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.Emit.RuntimeMethodBuilder">;
    use super::super::super::*;
    impl From<RuntimeMethodBuilder> for System::Reflection::Emit::MethodBuilder {
     fn from(v:RuntimeMethodBuilder)->System::Reflection::Emit::MethodBuilder{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::Emit::MethodBuilder,RuntimeMethodBuilder>(v)
    }} 
    pub type LocalSymInfo =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.Emit.LocalSymInfo">;
    use super::super::super::*;
    impl From<LocalSymInfo> for System::Object {
     fn from(v:LocalSymInfo)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,LocalSymInfo>(v)
    }} 
    pub type RuntimeModuleBuilder =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.Emit.RuntimeModuleBuilder">;
    use super::super::super::*;
    impl From<RuntimeModuleBuilder> for System::Reflection::Emit::ModuleBuilder {
     fn from(v:RuntimeModuleBuilder)->System::Reflection::Emit::ModuleBuilder{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::Emit::ModuleBuilder,RuntimeModuleBuilder>(v)
    }} 
    pub type RuntimeParameterBuilder =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.Emit.RuntimeParameterBuilder">;
    use super::super::super::*;
    impl From<RuntimeParameterBuilder> for System::Reflection::Emit::ParameterBuilder {
     fn from(v:RuntimeParameterBuilder)->System::Reflection::Emit::ParameterBuilder{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::Emit::ParameterBuilder,RuntimeParameterBuilder>(v)
    }} 
    pub type RuntimePropertyBuilder =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.Emit.RuntimePropertyBuilder">;
    use super::super::super::*;
    impl From<RuntimePropertyBuilder> for System::Reflection::Emit::PropertyBuilder {
     fn from(v:RuntimePropertyBuilder)->System::Reflection::Emit::PropertyBuilder{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::Emit::PropertyBuilder,RuntimePropertyBuilder>(v)
    }} 
    pub type TypeBuilder =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.Emit.TypeBuilder">;
    use super::super::super::*;
    impl From<TypeBuilder> for System::Reflection::TypeInfo {
     fn from(v:TypeBuilder)->System::Reflection::TypeInfo{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::TypeInfo,TypeBuilder>(v)
    }} 
    pub type RuntimeTypeBuilder =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.Emit.RuntimeTypeBuilder">;
    use super::super::super::*;
    impl From<RuntimeTypeBuilder> for System::Reflection::Emit::TypeBuilder {
     fn from(v:RuntimeTypeBuilder)->System::Reflection::Emit::TypeBuilder{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::Emit::TypeBuilder,RuntimeTypeBuilder>(v)
    }} 
    pub type SignatureHelper =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.Emit.SignatureHelper">;
    use super::super::super::*;
    impl From<SignatureHelper> for System::Object {
     fn from(v:SignatureHelper)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,SignatureHelper>(v)
    }} 
    pub type SymbolMethod =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.Emit.SymbolMethod">;
    use super::super::super::*;
    impl From<SymbolMethod> for System::Reflection::MethodInfo {
     fn from(v:SymbolMethod)->System::Reflection::MethodInfo{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::MethodInfo,SymbolMethod>(v)
    }} 
    pub type ILGenerator =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.Emit.ILGenerator">;
    use super::super::super::*;
    impl From<ILGenerator> for System::Object {
     fn from(v:ILGenerator)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,ILGenerator>(v)
    }} 
    pub type ConstructorBuilder =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.Emit.ConstructorBuilder">;
    use super::super::super::*;
    impl From<ConstructorBuilder> for System::Reflection::ConstructorInfo {
     fn from(v:ConstructorBuilder)->System::Reflection::ConstructorInfo{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::ConstructorInfo,ConstructorBuilder>(v)
    }} 
    pub type ConstructorOnTypeBuilderInstantiation =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.Emit.ConstructorOnTypeBuilderInstantiation">;
    use super::super::super::*;
    impl From<ConstructorOnTypeBuilderInstantiation> for System::Reflection::ConstructorInfo {
     fn from(v:ConstructorOnTypeBuilderInstantiation)->System::Reflection::ConstructorInfo{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::ConstructorInfo,ConstructorOnTypeBuilderInstantiation>(v)
    }} 
    pub type EmptyCAHolder =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.Emit.EmptyCAHolder">;
    use super::super::super::*;
    impl From<EmptyCAHolder> for System::Object {
     fn from(v:EmptyCAHolder)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,EmptyCAHolder>(v)
    }} 
    pub type EnumBuilder =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.Emit.EnumBuilder">;
    use super::super::super::*;
    impl From<EnumBuilder> for System::Reflection::TypeInfo {
     fn from(v:EnumBuilder)->System::Reflection::TypeInfo{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::TypeInfo,EnumBuilder>(v)
    }} 
    pub type EventBuilder =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.Emit.EventBuilder">;
    use super::super::super::*;
    impl From<EventBuilder> for System::Object {
     fn from(v:EventBuilder)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,EventBuilder>(v)
    }} 
    pub type FieldBuilder =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.Emit.FieldBuilder">;
    use super::super::super::*;
    impl From<FieldBuilder> for System::Reflection::FieldInfo {
     fn from(v:FieldBuilder)->System::Reflection::FieldInfo{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::FieldInfo,FieldBuilder>(v)
    }} 
    pub type FieldOnTypeBuilderInstantiation =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.Emit.FieldOnTypeBuilderInstantiation">;
    use super::super::super::*;
    impl From<FieldOnTypeBuilderInstantiation> for System::Reflection::FieldInfo {
     fn from(v:FieldOnTypeBuilderInstantiation)->System::Reflection::FieldInfo{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::FieldInfo,FieldOnTypeBuilderInstantiation>(v)
    }} 
    pub type GenericTypeParameterBuilder =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.Emit.GenericTypeParameterBuilder">;
    use super::super::super::*;
    impl From<GenericTypeParameterBuilder> for System::Reflection::TypeInfo {
     fn from(v:GenericTypeParameterBuilder)->System::Reflection::TypeInfo{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::TypeInfo,GenericTypeParameterBuilder>(v)
    }} 
    pub type MethodBuilder =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.Emit.MethodBuilder">;
    use super::super::super::*;
    impl From<MethodBuilder> for System::Reflection::MethodInfo {
     fn from(v:MethodBuilder)->System::Reflection::MethodInfo{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::MethodInfo,MethodBuilder>(v)
    }} 
    pub type MethodBuilderInstantiation =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.Emit.MethodBuilderInstantiation">;
    use super::super::super::*;
    impl From<MethodBuilderInstantiation> for System::Reflection::MethodInfo {
     fn from(v:MethodBuilderInstantiation)->System::Reflection::MethodInfo{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::MethodInfo,MethodBuilderInstantiation>(v)
    }} 
    pub type MethodOnTypeBuilderInstantiation =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.Emit.MethodOnTypeBuilderInstantiation">;
    use super::super::super::*;
    impl From<MethodOnTypeBuilderInstantiation> for System::Reflection::MethodInfo {
     fn from(v:MethodOnTypeBuilderInstantiation)->System::Reflection::MethodInfo{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::MethodInfo,MethodOnTypeBuilderInstantiation>(v)
    }} 
    pub type ModuleBuilder =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.Emit.ModuleBuilder">;
    use super::super::super::*;
    impl From<ModuleBuilder> for System::Reflection::Module {
     fn from(v:ModuleBuilder)->System::Reflection::Module{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::Module,ModuleBuilder>(v)
    }} 
    pub type OpCodes =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.Emit.OpCodes">;
    use super::super::super::*;
    impl From<OpCodes> for System::Object {
     fn from(v:OpCodes)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,OpCodes>(v)
    }} 
    pub type ParameterBuilder =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.Emit.ParameterBuilder">;
    use super::super::super::*;
    impl From<ParameterBuilder> for System::Object {
     fn from(v:ParameterBuilder)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,ParameterBuilder>(v)
    }} 
    pub type PropertyBuilder =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.Emit.PropertyBuilder">;
    use super::super::super::*;
    impl From<PropertyBuilder> for System::Reflection::PropertyInfo {
     fn from(v:PropertyBuilder)->System::Reflection::PropertyInfo{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::PropertyInfo,PropertyBuilder>(v)
    }} 
    pub type SymbolType =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.Emit.SymbolType">;
    use super::super::super::*;
    impl From<SymbolType> for System::Reflection::TypeInfo {
     fn from(v:SymbolType)->System::Reflection::TypeInfo{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::TypeInfo,SymbolType>(v)
    }} 
    pub type TypeBuilderInstantiation =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.Emit.TypeBuilderInstantiation">;
    use super::super::super::*;
    impl From<TypeBuilderInstantiation> for System::Reflection::TypeInfo {
     fn from(v:TypeBuilderInstantiation)->System::Reflection::TypeInfo{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::TypeInfo,TypeBuilderInstantiation>(v)
    }} 
    pub type TypeNameBuilder =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.Emit.TypeNameBuilder">;
    use super::super::super::*;
    impl From<TypeNameBuilder> for System::Object {
     fn from(v:TypeNameBuilder)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,TypeNameBuilder>(v)
    }} 
    }
    pub mod Metadata{
    pub type AssemblyExtensions =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.Metadata.AssemblyExtensions">;
    use super::super::super::*;
    impl From<AssemblyExtensions> for System::Object {
     fn from(v:AssemblyExtensions)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,AssemblyExtensions>(v)
    }} 
    pub type MetadataUpdater =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.Metadata.MetadataUpdater">;
    use super::super::super::*;
    impl From<MetadataUpdater> for System::Object {
     fn from(v:MetadataUpdater)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,MetadataUpdater>(v)
    }} 
    pub type RuntimeTypeMetadataUpdateHandler =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.Metadata.RuntimeTypeMetadataUpdateHandler">;
    use super::super::super::*;
    impl From<RuntimeTypeMetadataUpdateHandler> for System::Object {
     fn from(v:RuntimeTypeMetadataUpdateHandler)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,RuntimeTypeMetadataUpdateHandler>(v)
    }} 
    pub type MetadataUpdateHandlerAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.Metadata.MetadataUpdateHandlerAttribute">;
    use super::super::super::*;
    impl From<MetadataUpdateHandlerAttribute> for System::Attribute {
     fn from(v:MetadataUpdateHandlerAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,MetadataUpdateHandlerAttribute>(v)
    }} 
    }
    pub type Assembly =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.Assembly">;
    use super::super::*;
    impl From<Assembly> for System::Object {
     fn from(v:Assembly)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Assembly>(v)
    }} 
    pub type AssemblyName =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.AssemblyName">;
    use super::super::*;
    impl From<AssemblyName> for System::Object {
     fn from(v:AssemblyName)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,AssemblyName>(v)
    }} 
    pub type Associates =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.Associates">;
    use super::super::*;
    impl From<Associates> for System::Object {
     fn from(v:Associates)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Associates>(v)
    }} 
    pub type ConstructorInfo =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.ConstructorInfo">;
    use super::super::*;
    impl From<ConstructorInfo> for System::Reflection::MethodBase {
     fn from(v:ConstructorInfo)->System::Reflection::MethodBase{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::MethodBase,ConstructorInfo>(v)
    }} 
    pub type ConstructorInvoker =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.ConstructorInvoker">;
    use super::super::*;
    impl From<ConstructorInvoker> for System::Object {
     fn from(v:ConstructorInvoker)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,ConstructorInvoker>(v)
    }} 
    pub type FieldInfo =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.FieldInfo">;
    use super::super::*;
    impl From<FieldInfo> for System::Reflection::MemberInfo {
     fn from(v:FieldInfo)->System::Reflection::MemberInfo{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::MemberInfo,FieldInfo>(v)
    }} 
    pub type LoaderAllocatorScout =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.LoaderAllocatorScout">;
    use super::super::*;
    impl From<LoaderAllocatorScout> for System::Object {
     fn from(v:LoaderAllocatorScout)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,LoaderAllocatorScout>(v)
    }} 
    pub type LoaderAllocator =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.LoaderAllocator">;
    use super::super::*;
    impl From<LoaderAllocator> for System::Object {
     fn from(v:LoaderAllocator)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,LoaderAllocator>(v)
    }} 
    pub type MdConstant =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.MdConstant">;
    use super::super::*;
    impl From<MdConstant> for System::Object {
     fn from(v:MdConstant)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,MdConstant>(v)
    }} 
    pub type MdFieldInfo =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.MdFieldInfo">;
    use super::super::*;
    impl From<MdFieldInfo> for System::Reflection::RuntimeFieldInfo {
     fn from(v:MdFieldInfo)->System::Reflection::RuntimeFieldInfo{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::RuntimeFieldInfo,MdFieldInfo>(v)
    }} 
    pub type MetadataException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.MetadataException">;
    use super::super::*;
    impl From<MetadataException> for System::Exception {
     fn from(v:MetadataException)->System::Exception{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Exception,MetadataException>(v)
    }} 
    pub type MemberInfo =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.MemberInfo">;
    use super::super::*;
    impl From<MemberInfo> for System::Object {
     fn from(v:MemberInfo)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,MemberInfo>(v)
    }} 
    pub type MethodBase =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.MethodBase">;
    use super::super::*;
    impl From<MethodBase> for System::Reflection::MemberInfo {
     fn from(v:MethodBase)->System::Reflection::MemberInfo{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::MemberInfo,MethodBase>(v)
    }} 
    pub type MethodBaseInvoker =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.MethodBaseInvoker">;
    use super::super::*;
    impl From<MethodBaseInvoker> for System::Object {
     fn from(v:MethodBaseInvoker)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,MethodBaseInvoker>(v)
    }} 
    pub type MethodInvoker =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.MethodInvoker">;
    use super::super::*;
    impl From<MethodInvoker> for System::Object {
     fn from(v:MethodInvoker)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,MethodInvoker>(v)
    }} 
    pub type ModifiedType =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.ModifiedType">;
    use super::super::*;
    impl From<ModifiedType> for System::Type {
     fn from(v:ModifiedType)->System::Type{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Type,ModifiedType>(v)
    }} 
    pub type RtFieldInfo =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.RtFieldInfo">;
    use super::super::*;
    impl From<RtFieldInfo> for System::Reflection::RuntimeFieldInfo {
     fn from(v:RtFieldInfo)->System::Reflection::RuntimeFieldInfo{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::RuntimeFieldInfo,RtFieldInfo>(v)
    }} 
    pub type RuntimeAssembly =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.RuntimeAssembly">;
    use super::super::*;
    impl From<RuntimeAssembly> for System::Reflection::Assembly {
     fn from(v:RuntimeAssembly)->System::Reflection::Assembly{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::Assembly,RuntimeAssembly>(v)
    }} 
    pub type RuntimeConstructorInfo =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.RuntimeConstructorInfo">;
    use super::super::*;
    impl From<RuntimeConstructorInfo> for System::Reflection::ConstructorInfo {
     fn from(v:RuntimeConstructorInfo)->System::Reflection::ConstructorInfo{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::ConstructorInfo,RuntimeConstructorInfo>(v)
    }} 
    pub type RuntimeCustomAttributeData =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.RuntimeCustomAttributeData">;
    use super::super::*;
    impl From<RuntimeCustomAttributeData> for System::Reflection::CustomAttributeData {
     fn from(v:RuntimeCustomAttributeData)->System::Reflection::CustomAttributeData{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::CustomAttributeData,RuntimeCustomAttributeData>(v)
    }} 
    pub type CustomAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.CustomAttribute">;
    use super::super::*;
    impl From<CustomAttribute> for System::Object {
     fn from(v:CustomAttribute)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,CustomAttribute>(v)
    }} 
    pub type PseudoCustomAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.PseudoCustomAttribute">;
    use super::super::*;
    impl From<PseudoCustomAttribute> for System::Object {
     fn from(v:PseudoCustomAttribute)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,PseudoCustomAttribute>(v)
    }} 
    pub type RuntimeEventInfo =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.RuntimeEventInfo">;
    use super::super::*;
    impl From<RuntimeEventInfo> for System::Reflection::EventInfo {
     fn from(v:RuntimeEventInfo)->System::Reflection::EventInfo{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::EventInfo,RuntimeEventInfo>(v)
    }} 
    pub type RuntimeExceptionHandlingClause =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.RuntimeExceptionHandlingClause">;
    use super::super::*;
    impl From<RuntimeExceptionHandlingClause> for System::Reflection::ExceptionHandlingClause {
     fn from(v:RuntimeExceptionHandlingClause)->System::Reflection::ExceptionHandlingClause{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::ExceptionHandlingClause,RuntimeExceptionHandlingClause>(v)
    }} 
    pub type RuntimeFieldInfo =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.RuntimeFieldInfo">;
    use super::super::*;
    impl From<RuntimeFieldInfo> for System::Reflection::FieldInfo {
     fn from(v:RuntimeFieldInfo)->System::Reflection::FieldInfo{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::FieldInfo,RuntimeFieldInfo>(v)
    }} 
    pub type RuntimeLocalVariableInfo =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.RuntimeLocalVariableInfo">;
    use super::super::*;
    impl From<RuntimeLocalVariableInfo> for System::Reflection::LocalVariableInfo {
     fn from(v:RuntimeLocalVariableInfo)->System::Reflection::LocalVariableInfo{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::LocalVariableInfo,RuntimeLocalVariableInfo>(v)
    }} 
    pub type RuntimeMethodBody =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.RuntimeMethodBody">;
    use super::super::*;
    impl From<RuntimeMethodBody> for System::Reflection::MethodBody {
     fn from(v:RuntimeMethodBody)->System::Reflection::MethodBody{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::MethodBody,RuntimeMethodBody>(v)
    }} 
    pub type RuntimeMethodInfo =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.RuntimeMethodInfo">;
    use super::super::*;
    impl From<RuntimeMethodInfo> for System::Reflection::MethodInfo {
     fn from(v:RuntimeMethodInfo)->System::Reflection::MethodInfo{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::MethodInfo,RuntimeMethodInfo>(v)
    }} 
    pub type RuntimeModule =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.RuntimeModule">;
    use super::super::*;
    impl From<RuntimeModule> for System::Reflection::Module {
     fn from(v:RuntimeModule)->System::Reflection::Module{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::Module,RuntimeModule>(v)
    }} 
    pub type RuntimeParameterInfo =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.RuntimeParameterInfo">;
    use super::super::*;
    impl From<RuntimeParameterInfo> for System::Reflection::ParameterInfo {
     fn from(v:RuntimeParameterInfo)->System::Reflection::ParameterInfo{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::ParameterInfo,RuntimeParameterInfo>(v)
    }} 
    pub type RuntimePropertyInfo =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.RuntimePropertyInfo">;
    use super::super::*;
    impl From<RuntimePropertyInfo> for System::Reflection::PropertyInfo {
     fn from(v:RuntimePropertyInfo)->System::Reflection::PropertyInfo{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::PropertyInfo,RuntimePropertyInfo>(v)
    }} 
    pub type AmbiguousMatchException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.AmbiguousMatchException">;
    use super::super::*;
    impl From<AmbiguousMatchException> for System::SystemException {
     fn from(v:AmbiguousMatchException)->System::SystemException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::SystemException,AmbiguousMatchException>(v)
    }} 
    pub type AssemblyAlgorithmIdAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.AssemblyAlgorithmIdAttribute">;
    use super::super::*;
    impl From<AssemblyAlgorithmIdAttribute> for System::Attribute {
     fn from(v:AssemblyAlgorithmIdAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,AssemblyAlgorithmIdAttribute>(v)
    }} 
    pub type AssemblyCompanyAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.AssemblyCompanyAttribute">;
    use super::super::*;
    impl From<AssemblyCompanyAttribute> for System::Attribute {
     fn from(v:AssemblyCompanyAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,AssemblyCompanyAttribute>(v)
    }} 
    pub type AssemblyConfigurationAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.AssemblyConfigurationAttribute">;
    use super::super::*;
    impl From<AssemblyConfigurationAttribute> for System::Attribute {
     fn from(v:AssemblyConfigurationAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,AssemblyConfigurationAttribute>(v)
    }} 
    pub type AssemblyCopyrightAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.AssemblyCopyrightAttribute">;
    use super::super::*;
    impl From<AssemblyCopyrightAttribute> for System::Attribute {
     fn from(v:AssemblyCopyrightAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,AssemblyCopyrightAttribute>(v)
    }} 
    pub type AssemblyCultureAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.AssemblyCultureAttribute">;
    use super::super::*;
    impl From<AssemblyCultureAttribute> for System::Attribute {
     fn from(v:AssemblyCultureAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,AssemblyCultureAttribute>(v)
    }} 
    pub type AssemblyDefaultAliasAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.AssemblyDefaultAliasAttribute">;
    use super::super::*;
    impl From<AssemblyDefaultAliasAttribute> for System::Attribute {
     fn from(v:AssemblyDefaultAliasAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,AssemblyDefaultAliasAttribute>(v)
    }} 
    pub type AssemblyDelaySignAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.AssemblyDelaySignAttribute">;
    use super::super::*;
    impl From<AssemblyDelaySignAttribute> for System::Attribute {
     fn from(v:AssemblyDelaySignAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,AssemblyDelaySignAttribute>(v)
    }} 
    pub type AssemblyDescriptionAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.AssemblyDescriptionAttribute">;
    use super::super::*;
    impl From<AssemblyDescriptionAttribute> for System::Attribute {
     fn from(v:AssemblyDescriptionAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,AssemblyDescriptionAttribute>(v)
    }} 
    pub type AssemblyFileVersionAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.AssemblyFileVersionAttribute">;
    use super::super::*;
    impl From<AssemblyFileVersionAttribute> for System::Attribute {
     fn from(v:AssemblyFileVersionAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,AssemblyFileVersionAttribute>(v)
    }} 
    pub type AssemblyFlagsAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.AssemblyFlagsAttribute">;
    use super::super::*;
    impl From<AssemblyFlagsAttribute> for System::Attribute {
     fn from(v:AssemblyFlagsAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,AssemblyFlagsAttribute>(v)
    }} 
    pub type AssemblyInformationalVersionAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.AssemblyInformationalVersionAttribute">;
    use super::super::*;
    impl From<AssemblyInformationalVersionAttribute> for System::Attribute {
     fn from(v:AssemblyInformationalVersionAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,AssemblyInformationalVersionAttribute>(v)
    }} 
    pub type AssemblyKeyFileAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.AssemblyKeyFileAttribute">;
    use super::super::*;
    impl From<AssemblyKeyFileAttribute> for System::Attribute {
     fn from(v:AssemblyKeyFileAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,AssemblyKeyFileAttribute>(v)
    }} 
    pub type AssemblyKeyNameAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.AssemblyKeyNameAttribute">;
    use super::super::*;
    impl From<AssemblyKeyNameAttribute> for System::Attribute {
     fn from(v:AssemblyKeyNameAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,AssemblyKeyNameAttribute>(v)
    }} 
    pub type AssemblyMetadataAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.AssemblyMetadataAttribute">;
    use super::super::*;
    impl From<AssemblyMetadataAttribute> for System::Attribute {
     fn from(v:AssemblyMetadataAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,AssemblyMetadataAttribute>(v)
    }} 
    pub type AssemblyNameHelpers =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.AssemblyNameHelpers">;
    use super::super::*;
    impl From<AssemblyNameHelpers> for System::Object {
     fn from(v:AssemblyNameHelpers)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,AssemblyNameHelpers>(v)
    }} 
    pub type AssemblyNameFormatter =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.AssemblyNameFormatter">;
    use super::super::*;
    impl From<AssemblyNameFormatter> for System::Object {
     fn from(v:AssemblyNameFormatter)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,AssemblyNameFormatter>(v)
    }} 
    pub type AssemblyNameProxy =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.AssemblyNameProxy">;
    use super::super::*;
    impl From<AssemblyNameProxy> for System::MarshalByRefObject {
     fn from(v:AssemblyNameProxy)->System::MarshalByRefObject{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::MarshalByRefObject,AssemblyNameProxy>(v)
    }} 
    pub type AssemblyProductAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.AssemblyProductAttribute">;
    use super::super::*;
    impl From<AssemblyProductAttribute> for System::Attribute {
     fn from(v:AssemblyProductAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,AssemblyProductAttribute>(v)
    }} 
    pub type AssemblySignatureKeyAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.AssemblySignatureKeyAttribute">;
    use super::super::*;
    impl From<AssemblySignatureKeyAttribute> for System::Attribute {
     fn from(v:AssemblySignatureKeyAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,AssemblySignatureKeyAttribute>(v)
    }} 
    pub type AssemblyTitleAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.AssemblyTitleAttribute">;
    use super::super::*;
    impl From<AssemblyTitleAttribute> for System::Attribute {
     fn from(v:AssemblyTitleAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,AssemblyTitleAttribute>(v)
    }} 
    pub type AssemblyTrademarkAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.AssemblyTrademarkAttribute">;
    use super::super::*;
    impl From<AssemblyTrademarkAttribute> for System::Attribute {
     fn from(v:AssemblyTrademarkAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,AssemblyTrademarkAttribute>(v)
    }} 
    pub type AssemblyVersionAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.AssemblyVersionAttribute">;
    use super::super::*;
    impl From<AssemblyVersionAttribute> for System::Attribute {
     fn from(v:AssemblyVersionAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,AssemblyVersionAttribute>(v)
    }} 
    pub type Binder =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.Binder">;
    use super::super::*;
    impl From<Binder> for System::Object {
     fn from(v:Binder)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Binder>(v)
    }} 
    pub type CustomAttributeData =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.CustomAttributeData">;
    use super::super::*;
    impl From<CustomAttributeData> for System::Object {
     fn from(v:CustomAttributeData)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,CustomAttributeData>(v)
    }} 
    pub type CustomAttributeExtensions =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.CustomAttributeExtensions">;
    use super::super::*;
    impl From<CustomAttributeExtensions> for System::Object {
     fn from(v:CustomAttributeExtensions)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,CustomAttributeExtensions>(v)
    }} 
    pub type CustomAttributeFormatException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.CustomAttributeFormatException">;
    use super::super::*;
    impl From<CustomAttributeFormatException> for System::FormatException {
     fn from(v:CustomAttributeFormatException)->System::FormatException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::FormatException,CustomAttributeFormatException>(v)
    }} 
    pub type DefaultMemberAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.DefaultMemberAttribute">;
    use super::super::*;
    impl From<DefaultMemberAttribute> for System::Attribute {
     fn from(v:DefaultMemberAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,DefaultMemberAttribute>(v)
    }} 
    pub type EventInfo =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.EventInfo">;
    use super::super::*;
    impl From<EventInfo> for System::Reflection::MemberInfo {
     fn from(v:EventInfo)->System::Reflection::MemberInfo{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::MemberInfo,EventInfo>(v)
    }} 
    pub type ExceptionHandlingClause =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.ExceptionHandlingClause">;
    use super::super::*;
    impl From<ExceptionHandlingClause> for System::Object {
     fn from(v:ExceptionHandlingClause)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,ExceptionHandlingClause>(v)
    }} 
    pub type ICustomAttributeProvider =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.ICustomAttributeProvider">;
    use super::super::*;
    pub type IntrospectionExtensions =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.IntrospectionExtensions">;
    use super::super::*;
    impl From<IntrospectionExtensions> for System::Object {
     fn from(v:IntrospectionExtensions)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,IntrospectionExtensions>(v)
    }} 
    pub type InvalidFilterCriteriaException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.InvalidFilterCriteriaException">;
    use super::super::*;
    impl From<InvalidFilterCriteriaException> for System::ApplicationException {
     fn from(v:InvalidFilterCriteriaException)->System::ApplicationException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::ApplicationException,InvalidFilterCriteriaException>(v)
    }} 
    pub type InvokerEmitUtil =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.InvokerEmitUtil">;
    use super::super::*;
    impl From<InvokerEmitUtil> for System::Object {
     fn from(v:InvokerEmitUtil)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,InvokerEmitUtil>(v)
    }} 
    pub type InvokeUtils =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.InvokeUtils">;
    use super::super::*;
    impl From<InvokeUtils> for System::Object {
     fn from(v:InvokeUtils)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,InvokeUtils>(v)
    }} 
    pub type IReflect =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.IReflect">;
    use super::super::*;
    pub type IReflectableType =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.IReflectableType">;
    use super::super::*;
    pub type LocalVariableInfo =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.LocalVariableInfo">;
    use super::super::*;
    impl From<LocalVariableInfo> for System::Object {
     fn from(v:LocalVariableInfo)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,LocalVariableInfo>(v)
    }} 
    pub type ManifestResourceInfo =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.ManifestResourceInfo">;
    use super::super::*;
    impl From<ManifestResourceInfo> for System::Object {
     fn from(v:ManifestResourceInfo)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,ManifestResourceInfo>(v)
    }} 
    pub type MemberFilter =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.MemberFilter">;
    use super::super::*;
    impl From<MemberFilter> for System::MulticastDelegate {
     fn from(v:MemberFilter)->System::MulticastDelegate{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::MulticastDelegate,MemberFilter>(v)
    }} 
    pub type MethodBody =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.MethodBody">;
    use super::super::*;
    impl From<MethodBody> for System::Object {
     fn from(v:MethodBody)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,MethodBody>(v)
    }} 
    pub type MethodInfo =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.MethodInfo">;
    use super::super::*;
    impl From<MethodInfo> for System::Reflection::MethodBase {
     fn from(v:MethodInfo)->System::Reflection::MethodBase{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::MethodBase,MethodInfo>(v)
    }} 
    pub type MethodInvokerCommon =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.MethodInvokerCommon">;
    use super::super::*;
    impl From<MethodInvokerCommon> for System::Object {
     fn from(v:MethodInvokerCommon)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,MethodInvokerCommon>(v)
    }} 
    pub type Missing =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.Missing">;
    use super::super::*;
    impl From<Missing> for System::Object {
     fn from(v:Missing)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Missing>(v)
    }} 
    pub type ModifiedHasElementType =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.ModifiedHasElementType">;
    use super::super::*;
    impl From<ModifiedHasElementType> for System::Reflection::ModifiedType {
     fn from(v:ModifiedHasElementType)->System::Reflection::ModifiedType{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::ModifiedType,ModifiedHasElementType>(v)
    }} 
    pub type ModifiedFunctionPointerType =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.ModifiedFunctionPointerType">;
    use super::super::*;
    impl From<ModifiedFunctionPointerType> for System::Reflection::ModifiedType {
     fn from(v:ModifiedFunctionPointerType)->System::Reflection::ModifiedType{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::ModifiedType,ModifiedFunctionPointerType>(v)
    }} 
    pub type ModifiedGenericType =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.ModifiedGenericType">;
    use super::super::*;
    impl From<ModifiedGenericType> for System::Reflection::ModifiedType {
     fn from(v:ModifiedGenericType)->System::Reflection::ModifiedType{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::ModifiedType,ModifiedGenericType>(v)
    }} 
    pub type Module =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.Module">;
    use super::super::*;
    impl From<Module> for System::Object {
     fn from(v:Module)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Module>(v)
    }} 
    pub type ModuleResolveEventHandler =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.ModuleResolveEventHandler">;
    use super::super::*;
    impl From<ModuleResolveEventHandler> for System::MulticastDelegate {
     fn from(v:ModuleResolveEventHandler)->System::MulticastDelegate{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::MulticastDelegate,ModuleResolveEventHandler>(v)
    }} 
    pub type NullabilityInfo =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.NullabilityInfo">;
    use super::super::*;
    impl From<NullabilityInfo> for System::Object {
     fn from(v:NullabilityInfo)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,NullabilityInfo>(v)
    }} 
    pub type NullabilityInfoContext =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.NullabilityInfoContext">;
    use super::super::*;
    impl From<NullabilityInfoContext> for System::Object {
     fn from(v:NullabilityInfoContext)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,NullabilityInfoContext>(v)
    }} 
    pub type ObfuscateAssemblyAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.ObfuscateAssemblyAttribute">;
    use super::super::*;
    impl From<ObfuscateAssemblyAttribute> for System::Attribute {
     fn from(v:ObfuscateAssemblyAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,ObfuscateAssemblyAttribute>(v)
    }} 
    pub type ObfuscationAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.ObfuscationAttribute">;
    use super::super::*;
    impl From<ObfuscationAttribute> for System::Attribute {
     fn from(v:ObfuscationAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,ObfuscationAttribute>(v)
    }} 
    pub type ParameterInfo =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.ParameterInfo">;
    use super::super::*;
    impl From<ParameterInfo> for System::Object {
     fn from(v:ParameterInfo)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,ParameterInfo>(v)
    }} 
    pub type Pointer =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.Pointer">;
    use super::super::*;
    impl From<Pointer> for System::Object {
     fn from(v:Pointer)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Pointer>(v)
    }} 
    pub type PropertyInfo =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.PropertyInfo">;
    use super::super::*;
    impl From<PropertyInfo> for System::Reflection::MemberInfo {
     fn from(v:PropertyInfo)->System::Reflection::MemberInfo{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::MemberInfo,PropertyInfo>(v)
    }} 
    pub type ReflectionContext =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.ReflectionContext">;
    use super::super::*;
    impl From<ReflectionContext> for System::Object {
     fn from(v:ReflectionContext)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,ReflectionContext>(v)
    }} 
    pub type ReflectionTypeLoadException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.ReflectionTypeLoadException">;
    use super::super::*;
    impl From<ReflectionTypeLoadException> for System::SystemException {
     fn from(v:ReflectionTypeLoadException)->System::SystemException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::SystemException,ReflectionTypeLoadException>(v)
    }} 
    pub type RuntimeReflectionExtensions =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.RuntimeReflectionExtensions">;
    use super::super::*;
    impl From<RuntimeReflectionExtensions> for System::Object {
     fn from(v:RuntimeReflectionExtensions)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,RuntimeReflectionExtensions>(v)
    }} 
    pub type SignatureArrayType =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.SignatureArrayType">;
    use super::super::*;
    impl From<SignatureArrayType> for System::Reflection::SignatureHasElementType {
     fn from(v:SignatureArrayType)->System::Reflection::SignatureHasElementType{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::SignatureHasElementType,SignatureArrayType>(v)
    }} 
    pub type SignatureByRefType =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.SignatureByRefType">;
    use super::super::*;
    impl From<SignatureByRefType> for System::Reflection::SignatureHasElementType {
     fn from(v:SignatureByRefType)->System::Reflection::SignatureHasElementType{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::SignatureHasElementType,SignatureByRefType>(v)
    }} 
    pub type SignatureConstructedGenericType =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.SignatureConstructedGenericType">;
    use super::super::*;
    impl From<SignatureConstructedGenericType> for System::Reflection::SignatureType {
     fn from(v:SignatureConstructedGenericType)->System::Reflection::SignatureType{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::SignatureType,SignatureConstructedGenericType>(v)
    }} 
    pub type SignatureGenericMethodParameterType =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.SignatureGenericMethodParameterType">;
    use super::super::*;
    impl From<SignatureGenericMethodParameterType> for System::Reflection::SignatureGenericParameterType {
     fn from(v:SignatureGenericMethodParameterType)->System::Reflection::SignatureGenericParameterType{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::SignatureGenericParameterType,SignatureGenericMethodParameterType>(v)
    }} 
    pub type SignatureGenericParameterType =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.SignatureGenericParameterType">;
    use super::super::*;
    impl From<SignatureGenericParameterType> for System::Reflection::SignatureType {
     fn from(v:SignatureGenericParameterType)->System::Reflection::SignatureType{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::SignatureType,SignatureGenericParameterType>(v)
    }} 
    pub type SignatureHasElementType =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.SignatureHasElementType">;
    use super::super::*;
    impl From<SignatureHasElementType> for System::Reflection::SignatureType {
     fn from(v:SignatureHasElementType)->System::Reflection::SignatureType{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::SignatureType,SignatureHasElementType>(v)
    }} 
    pub type SignaturePointerType =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.SignaturePointerType">;
    use super::super::*;
    impl From<SignaturePointerType> for System::Reflection::SignatureHasElementType {
     fn from(v:SignaturePointerType)->System::Reflection::SignatureHasElementType{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::SignatureHasElementType,SignaturePointerType>(v)
    }} 
    pub type SignatureType =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.SignatureType">;
    use super::super::*;
    impl From<SignatureType> for System::Type {
     fn from(v:SignatureType)->System::Type{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Type,SignatureType>(v)
    }} 
    pub type SignatureTypeExtensions =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.SignatureTypeExtensions">;
    use super::super::*;
    impl From<SignatureTypeExtensions> for System::Object {
     fn from(v:SignatureTypeExtensions)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,SignatureTypeExtensions>(v)
    }} 
    pub type StrongNameKeyPair =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.StrongNameKeyPair">;
    use super::super::*;
    impl From<StrongNameKeyPair> for System::Object {
     fn from(v:StrongNameKeyPair)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,StrongNameKeyPair>(v)
    }} 
    pub type TargetException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.TargetException">;
    use super::super::*;
    impl From<TargetException> for System::ApplicationException {
     fn from(v:TargetException)->System::ApplicationException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::ApplicationException,TargetException>(v)
    }} 
    pub type TargetInvocationException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.TargetInvocationException">;
    use super::super::*;
    impl From<TargetInvocationException> for System::ApplicationException {
     fn from(v:TargetInvocationException)->System::ApplicationException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::ApplicationException,TargetInvocationException>(v)
    }} 
    pub type TargetParameterCountException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.TargetParameterCountException">;
    use super::super::*;
    impl From<TargetParameterCountException> for System::ApplicationException {
     fn from(v:TargetParameterCountException)->System::ApplicationException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::ApplicationException,TargetParameterCountException>(v)
    }} 
    pub type TypeDelegator =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.TypeDelegator">;
    use super::super::*;
    impl From<TypeDelegator> for System::Reflection::TypeInfo {
     fn from(v:TypeDelegator)->System::Reflection::TypeInfo{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::TypeInfo,TypeDelegator>(v)
    }} 
    pub type TypeFilter =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.TypeFilter">;
    use super::super::*;
    impl From<TypeFilter> for System::MulticastDelegate {
     fn from(v:TypeFilter)->System::MulticastDelegate{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::MulticastDelegate,TypeFilter>(v)
    }} 
    pub type TypeInfo =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Reflection.TypeInfo">;
    use super::super::*;
    impl From<TypeInfo> for System::Type {
     fn from(v:TypeInfo)->System::Type{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Type,TypeInfo>(v)
    }} 
    }
    pub mod Globalization{
    pub type Calendar =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Globalization.Calendar">;
    use super::super::*;
    impl From<Calendar> for System::Object {
     fn from(v:Calendar)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Calendar>(v)
    }} 
    pub type CalendarData =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Globalization.CalendarData">;
    use super::super::*;
    impl From<CalendarData> for System::Object {
     fn from(v:CalendarData)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,CalendarData>(v)
    }} 
    pub type CalendricalCalculationsHelper =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Globalization.CalendricalCalculationsHelper">;
    use super::super::*;
    impl From<CalendricalCalculationsHelper> for System::Object {
     fn from(v:CalendricalCalculationsHelper)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,CalendricalCalculationsHelper>(v)
    }} 
    pub type CharUnicodeInfo =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Globalization.CharUnicodeInfo">;
    use super::super::*;
    impl From<CharUnicodeInfo> for System::Object {
     fn from(v:CharUnicodeInfo)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,CharUnicodeInfo>(v)
    }} 
    pub type ChineseLunisolarCalendar =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Globalization.ChineseLunisolarCalendar">;
    use super::super::*;
    impl From<ChineseLunisolarCalendar> for System::Globalization::EastAsianLunisolarCalendar {
     fn from(v:ChineseLunisolarCalendar)->System::Globalization::EastAsianLunisolarCalendar{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Globalization::EastAsianLunisolarCalendar,ChineseLunisolarCalendar>(v)
    }} 
    pub type CompareInfo =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Globalization.CompareInfo">;
    use super::super::*;
    impl From<CompareInfo> for System::Object {
     fn from(v:CompareInfo)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,CompareInfo>(v)
    }} 
    pub type CultureData =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Globalization.CultureData">;
    use super::super::*;
    impl From<CultureData> for System::Object {
     fn from(v:CultureData)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,CultureData>(v)
    }} 
    pub type CultureInfo =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Globalization.CultureInfo">;
    use super::super::*;
    impl From<CultureInfo> for System::Object {
     fn from(v:CultureInfo)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,CultureInfo>(v)
    }} 
    pub type CultureNotFoundException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Globalization.CultureNotFoundException">;
    use super::super::*;
    impl From<CultureNotFoundException> for System::ArgumentException {
     fn from(v:CultureNotFoundException)->System::ArgumentException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::ArgumentException,CultureNotFoundException>(v)
    }} 
    pub type DateTimeFormatInfo =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Globalization.DateTimeFormatInfo">;
    use super::super::*;
    impl From<DateTimeFormatInfo> for System::Object {
     fn from(v:DateTimeFormatInfo)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,DateTimeFormatInfo>(v)
    }} 
    pub type DateTimeFormatInfoScanner =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Globalization.DateTimeFormatInfoScanner">;
    use super::super::*;
    impl From<DateTimeFormatInfoScanner> for System::Object {
     fn from(v:DateTimeFormatInfoScanner)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,DateTimeFormatInfoScanner>(v)
    }} 
    pub type DaylightTime =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Globalization.DaylightTime">;
    use super::super::*;
    impl From<DaylightTime> for System::Object {
     fn from(v:DaylightTime)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,DaylightTime>(v)
    }} 
    pub type EastAsianLunisolarCalendar =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Globalization.EastAsianLunisolarCalendar">;
    use super::super::*;
    impl From<EastAsianLunisolarCalendar> for System::Globalization::Calendar {
     fn from(v:EastAsianLunisolarCalendar)->System::Globalization::Calendar{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Globalization::Calendar,EastAsianLunisolarCalendar>(v)
    }} 
    pub type GlobalizationExtensions =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Globalization.GlobalizationExtensions">;
    use super::super::*;
    impl From<GlobalizationExtensions> for System::Object {
     fn from(v:GlobalizationExtensions)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,GlobalizationExtensions>(v)
    }} 
    pub type GlobalizationMode =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Globalization.GlobalizationMode">;
    use super::super::*;
    impl From<GlobalizationMode> for System::Object {
     fn from(v:GlobalizationMode)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,GlobalizationMode>(v)
    }} 
    pub type GregorianCalendar =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Globalization.GregorianCalendar">;
    use super::super::*;
    impl From<GregorianCalendar> for System::Globalization::Calendar {
     fn from(v:GregorianCalendar)->System::Globalization::Calendar{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Globalization::Calendar,GregorianCalendar>(v)
    }} 
    pub type EraInfo =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Globalization.EraInfo">;
    use super::super::*;
    impl From<EraInfo> for System::Object {
     fn from(v:EraInfo)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,EraInfo>(v)
    }} 
    pub type GregorianCalendarHelper =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Globalization.GregorianCalendarHelper">;
    use super::super::*;
    impl From<GregorianCalendarHelper> for System::Object {
     fn from(v:GregorianCalendarHelper)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,GregorianCalendarHelper>(v)
    }} 
    pub type HebrewCalendar =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Globalization.HebrewCalendar">;
    use super::super::*;
    impl From<HebrewCalendar> for System::Globalization::Calendar {
     fn from(v:HebrewCalendar)->System::Globalization::Calendar{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Globalization::Calendar,HebrewCalendar>(v)
    }} 
    pub type HebrewNumber =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Globalization.HebrewNumber">;
    use super::super::*;
    impl From<HebrewNumber> for System::Object {
     fn from(v:HebrewNumber)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,HebrewNumber>(v)
    }} 
    pub type HijriCalendar =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Globalization.HijriCalendar">;
    use super::super::*;
    impl From<HijriCalendar> for System::Globalization::Calendar {
     fn from(v:HijriCalendar)->System::Globalization::Calendar{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Globalization::Calendar,HijriCalendar>(v)
    }} 
    pub type IcuLocaleData =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Globalization.IcuLocaleData">;
    use super::super::*;
    impl From<IcuLocaleData> for System::Object {
     fn from(v:IcuLocaleData)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,IcuLocaleData>(v)
    }} 
    pub type IdnMapping =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Globalization.IdnMapping">;
    use super::super::*;
    impl From<IdnMapping> for System::Object {
     fn from(v:IdnMapping)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,IdnMapping>(v)
    }} 
    pub type InvariantModeCasing =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Globalization.InvariantModeCasing">;
    use super::super::*;
    impl From<InvariantModeCasing> for System::Object {
     fn from(v:InvariantModeCasing)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,InvariantModeCasing>(v)
    }} 
    pub type ISOWeek =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Globalization.ISOWeek">;
    use super::super::*;
    impl From<ISOWeek> for System::Object {
     fn from(v:ISOWeek)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,ISOWeek>(v)
    }} 
    pub type JapaneseCalendar =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Globalization.JapaneseCalendar">;
    use super::super::*;
    impl From<JapaneseCalendar> for System::Globalization::Calendar {
     fn from(v:JapaneseCalendar)->System::Globalization::Calendar{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Globalization::Calendar,JapaneseCalendar>(v)
    }} 
    pub type JapaneseLunisolarCalendar =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Globalization.JapaneseLunisolarCalendar">;
    use super::super::*;
    impl From<JapaneseLunisolarCalendar> for System::Globalization::EastAsianLunisolarCalendar {
     fn from(v:JapaneseLunisolarCalendar)->System::Globalization::EastAsianLunisolarCalendar{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Globalization::EastAsianLunisolarCalendar,JapaneseLunisolarCalendar>(v)
    }} 
    pub type JulianCalendar =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Globalization.JulianCalendar">;
    use super::super::*;
    impl From<JulianCalendar> for System::Globalization::Calendar {
     fn from(v:JulianCalendar)->System::Globalization::Calendar{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Globalization::Calendar,JulianCalendar>(v)
    }} 
    pub type KoreanCalendar =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Globalization.KoreanCalendar">;
    use super::super::*;
    impl From<KoreanCalendar> for System::Globalization::Calendar {
     fn from(v:KoreanCalendar)->System::Globalization::Calendar{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Globalization::Calendar,KoreanCalendar>(v)
    }} 
    pub type KoreanLunisolarCalendar =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Globalization.KoreanLunisolarCalendar">;
    use super::super::*;
    impl From<KoreanLunisolarCalendar> for System::Globalization::EastAsianLunisolarCalendar {
     fn from(v:KoreanLunisolarCalendar)->System::Globalization::EastAsianLunisolarCalendar{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Globalization::EastAsianLunisolarCalendar,KoreanLunisolarCalendar>(v)
    }} 
    pub type Normalization =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Globalization.Normalization">;
    use super::super::*;
    impl From<Normalization> for System::Object {
     fn from(v:Normalization)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Normalization>(v)
    }} 
    pub type NumberFormatInfo =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Globalization.NumberFormatInfo">;
    use super::super::*;
    impl From<NumberFormatInfo> for System::Object {
     fn from(v:NumberFormatInfo)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,NumberFormatInfo>(v)
    }} 
    pub type Ordinal =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Globalization.Ordinal">;
    use super::super::*;
    impl From<Ordinal> for System::Object {
     fn from(v:Ordinal)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Ordinal>(v)
    }} 
    pub type OrdinalCasing =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Globalization.OrdinalCasing">;
    use super::super::*;
    impl From<OrdinalCasing> for System::Object {
     fn from(v:OrdinalCasing)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,OrdinalCasing>(v)
    }} 
    pub type PersianCalendar =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Globalization.PersianCalendar">;
    use super::super::*;
    impl From<PersianCalendar> for System::Globalization::Calendar {
     fn from(v:PersianCalendar)->System::Globalization::Calendar{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Globalization::Calendar,PersianCalendar>(v)
    }} 
    pub type RegionInfo =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Globalization.RegionInfo">;
    use super::super::*;
    impl From<RegionInfo> for System::Object {
     fn from(v:RegionInfo)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,RegionInfo>(v)
    }} 
    pub type SortKey =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Globalization.SortKey">;
    use super::super::*;
    impl From<SortKey> for System::Object {
     fn from(v:SortKey)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,SortKey>(v)
    }} 
    pub type SortVersion =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Globalization.SortVersion">;
    use super::super::*;
    impl From<SortVersion> for System::Object {
     fn from(v:SortVersion)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,SortVersion>(v)
    }} 
    pub type StringInfo =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Globalization.StringInfo">;
    use super::super::*;
    impl From<StringInfo> for System::Object {
     fn from(v:StringInfo)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,StringInfo>(v)
    }} 
    pub type SurrogateCasing =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Globalization.SurrogateCasing">;
    use super::super::*;
    impl From<SurrogateCasing> for System::Object {
     fn from(v:SurrogateCasing)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,SurrogateCasing>(v)
    }} 
    pub type TaiwanCalendar =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Globalization.TaiwanCalendar">;
    use super::super::*;
    impl From<TaiwanCalendar> for System::Globalization::Calendar {
     fn from(v:TaiwanCalendar)->System::Globalization::Calendar{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Globalization::Calendar,TaiwanCalendar>(v)
    }} 
    pub type TaiwanLunisolarCalendar =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Globalization.TaiwanLunisolarCalendar">;
    use super::super::*;
    impl From<TaiwanLunisolarCalendar> for System::Globalization::EastAsianLunisolarCalendar {
     fn from(v:TaiwanLunisolarCalendar)->System::Globalization::EastAsianLunisolarCalendar{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Globalization::EastAsianLunisolarCalendar,TaiwanLunisolarCalendar>(v)
    }} 
    pub type TextElementEnumerator =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Globalization.TextElementEnumerator">;
    use super::super::*;
    impl From<TextElementEnumerator> for System::Object {
     fn from(v:TextElementEnumerator)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,TextElementEnumerator>(v)
    }} 
    pub type TextInfo =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Globalization.TextInfo">;
    use super::super::*;
    impl From<TextInfo> for System::Object {
     fn from(v:TextInfo)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,TextInfo>(v)
    }} 
    pub type ThaiBuddhistCalendar =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Globalization.ThaiBuddhistCalendar">;
    use super::super::*;
    impl From<ThaiBuddhistCalendar> for System::Globalization::Calendar {
     fn from(v:ThaiBuddhistCalendar)->System::Globalization::Calendar{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Globalization::Calendar,ThaiBuddhistCalendar>(v)
    }} 
    pub type TimeSpanFormat =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Globalization.TimeSpanFormat">;
    use super::super::*;
    impl From<TimeSpanFormat> for System::Object {
     fn from(v:TimeSpanFormat)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,TimeSpanFormat>(v)
    }} 
    pub type TimeSpanParse =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Globalization.TimeSpanParse">;
    use super::super::*;
    impl From<TimeSpanParse> for System::Object {
     fn from(v:TimeSpanParse)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,TimeSpanParse>(v)
    }} 
    pub type UmAlQuraCalendar =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Globalization.UmAlQuraCalendar">;
    use super::super::*;
    impl From<UmAlQuraCalendar> for System::Globalization::Calendar {
     fn from(v:UmAlQuraCalendar)->System::Globalization::Calendar{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Globalization::Calendar,UmAlQuraCalendar>(v)
    }} 
    }
    pub type __Canon =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.__Canon">;
    use super::*;
    impl From<__Canon> for System::Object {
     fn from(v:__Canon)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,__Canon>(v)
    }} 
    pub type Array =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Array">;
    use super::*;
    impl From<Array> for System::Object {
     fn from(v:Array)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Array>(v)
    }} 
    pub type SZArrayHelper =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.SZArrayHelper">;
    use super::*;
    impl From<SZArrayHelper> for System::Object {
     fn from(v:SZArrayHelper)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,SZArrayHelper>(v)
    }} 
    pub type Attribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Attribute">;
    use super::*;
    impl From<Attribute> for System::Object {
     fn from(v:Attribute)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Attribute>(v)
    }} 
    pub type BadImageFormatException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.BadImageFormatException">;
    use super::*;
    impl From<BadImageFormatException> for System::SystemException {
     fn from(v:BadImageFormatException)->System::SystemException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::SystemException,BadImageFormatException>(v)
    }} 
    pub type Buffer =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Buffer">;
    use super::*;
    impl From<Buffer> for System::Object {
     fn from(v:Buffer)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Buffer>(v)
    }} 
    pub type ComAwareWeakReference =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.ComAwareWeakReference">;
    use super::*;
    impl From<ComAwareWeakReference> for System::Object {
     fn from(v:ComAwareWeakReference)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,ComAwareWeakReference>(v)
    }} 
    pub type Delegate =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Delegate">;
    use super::*;
    impl From<Delegate> for System::Object {
     fn from(v:Delegate)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Delegate>(v)
    }} 
    pub type Enum =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Enum">;
    use super::*;
    impl From<Enum> for System::ValueType {
     fn from(v:Enum)->System::ValueType{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::ValueType,Enum>(v)
    }} 
    pub type Environment =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Environment">;
    use super::*;
    impl From<Environment> for System::Object {
     fn from(v:Environment)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Environment>(v)
    }} 
    pub type Exception =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Exception">;
    use super::*;
    impl From<Exception> for System::Object {
     fn from(v:Exception)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Exception>(v)
    }} 
    pub type GC =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.GC">;
    use super::*;
    impl From<GC> for System::Object {
     fn from(v:GC)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,GC>(v)
    }} 
    pub type Math =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Math">;
    use super::*;
    impl From<Math> for System::Object {
     fn from(v:Math)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Math>(v)
    }} 
    pub type MathF =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.MathF">;
    use super::*;
    impl From<MathF> for System::Object {
     fn from(v:MathF)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,MathF>(v)
    }} 
    pub type MulticastDelegate =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.MulticastDelegate">;
    use super::*;
    impl From<MulticastDelegate> for System::Delegate {
     fn from(v:MulticastDelegate)->System::Delegate{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Delegate,MulticastDelegate>(v)
    }} 
    pub type Object =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Object">;
    use super::*;
    pub type RuntimeMethodInfoStub =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.RuntimeMethodInfoStub">;
    use super::*;
    impl From<RuntimeMethodInfoStub> for System::Object {
     fn from(v:RuntimeMethodInfoStub)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,RuntimeMethodInfoStub>(v)
    }} 
    pub type IRuntimeMethodInfo =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.IRuntimeMethodInfo">;
    use super::*;
    pub type IRuntimeFieldInfo =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.IRuntimeFieldInfo">;
    use super::*;
    pub type RuntimeFieldInfoStub =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.RuntimeFieldInfoStub">;
    use super::*;
    impl From<RuntimeFieldInfoStub> for System::Object {
     fn from(v:RuntimeFieldInfoStub)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,RuntimeFieldInfoStub>(v)
    }} 
    pub type Signature =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Signature">;
    use super::*;
    impl From<Signature> for System::Object {
     fn from(v:Signature)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Signature>(v)
    }} 
    pub type Resolver =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Resolver">;
    use super::*;
    impl From<Resolver> for System::Object {
     fn from(v:Resolver)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Resolver>(v)
    }} 
    pub type RuntimeType =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.RuntimeType">;
    use super::*;
    impl From<RuntimeType> for System::Reflection::TypeInfo {
     fn from(v:RuntimeType)->System::Reflection::TypeInfo{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::TypeInfo,RuntimeType>(v)
    }} 
    pub type StartupHookProvider =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.StartupHookProvider">;
    use super::*;
    impl From<StartupHookProvider> for System::Object {
     fn from(v:StartupHookProvider)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,StartupHookProvider>(v)
    }} 
    pub type String =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.String">;
    use super::*;
    impl From<String> for System::Object {
     fn from(v:String)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,String>(v)
    }} 
    pub type Type =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Type">;
    use super::*;
    impl From<Type> for System::Reflection::MemberInfo {
     fn from(v:Type)->System::Reflection::MemberInfo{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::MemberInfo,Type>(v)
    }} 
    pub type TypeLoadException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.TypeLoadException">;
    use super::*;
    impl From<TypeLoadException> for System::SystemException {
     fn from(v:TypeLoadException)->System::SystemException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::SystemException,TypeLoadException>(v)
    }} 
    pub type ValueType =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.ValueType">;
    use super::*;
    impl From<ValueType> for System::Object {
     fn from(v:ValueType)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,ValueType>(v)
    }} 
    pub type AccessViolationException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.AccessViolationException">;
    use super::*;
    impl From<AccessViolationException> for System::SystemException {
     fn from(v:AccessViolationException)->System::SystemException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::SystemException,AccessViolationException>(v)
    }} 
    pub type Action =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Action">;
    use super::*;
    impl From<Action> for System::MulticastDelegate {
     fn from(v:Action)->System::MulticastDelegate{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::MulticastDelegate,Action>(v)
    }} 
    pub type Activator =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Activator">;
    use super::*;
    impl From<Activator> for System::Object {
     fn from(v:Activator)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Activator>(v)
    }} 
    pub type AggregateException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.AggregateException">;
    use super::*;
    impl From<AggregateException> for System::Exception {
     fn from(v:AggregateException)->System::Exception{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Exception,AggregateException>(v)
    }} 
    pub type AppContext =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.AppContext">;
    use super::*;
    impl From<AppContext> for System::Object {
     fn from(v:AppContext)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,AppContext>(v)
    }} 
    pub type AppContextConfigHelper =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.AppContextConfigHelper">;
    use super::*;
    impl From<AppContextConfigHelper> for System::Object {
     fn from(v:AppContextConfigHelper)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,AppContextConfigHelper>(v)
    }} 
    pub type AppDomain =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.AppDomain">;
    use super::*;
    impl From<AppDomain> for System::MarshalByRefObject {
     fn from(v:AppDomain)->System::MarshalByRefObject{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::MarshalByRefObject,AppDomain>(v)
    }} 
    pub type AppDomainSetup =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.AppDomainSetup">;
    use super::*;
    impl From<AppDomainSetup> for System::Object {
     fn from(v:AppDomainSetup)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,AppDomainSetup>(v)
    }} 
    pub type AppDomainUnloadedException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.AppDomainUnloadedException">;
    use super::*;
    impl From<AppDomainUnloadedException> for System::SystemException {
     fn from(v:AppDomainUnloadedException)->System::SystemException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::SystemException,AppDomainUnloadedException>(v)
    }} 
    pub type ApplicationException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.ApplicationException">;
    use super::*;
    impl From<ApplicationException> for System::Exception {
     fn from(v:ApplicationException)->System::Exception{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Exception,ApplicationException>(v)
    }} 
    pub type ApplicationId =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.ApplicationId">;
    use super::*;
    impl From<ApplicationId> for System::Object {
     fn from(v:ApplicationId)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,ApplicationId>(v)
    }} 
    pub type ArgumentException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.ArgumentException">;
    use super::*;
    impl From<ArgumentException> for System::SystemException {
     fn from(v:ArgumentException)->System::SystemException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::SystemException,ArgumentException>(v)
    }} 
    pub type ArgumentNullException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.ArgumentNullException">;
    use super::*;
    impl From<ArgumentNullException> for System::ArgumentException {
     fn from(v:ArgumentNullException)->System::ArgumentException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::ArgumentException,ArgumentNullException>(v)
    }} 
    pub type ArgumentOutOfRangeException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.ArgumentOutOfRangeException">;
    use super::*;
    impl From<ArgumentOutOfRangeException> for System::ArgumentException {
     fn from(v:ArgumentOutOfRangeException)->System::ArgumentException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::ArgumentException,ArgumentOutOfRangeException>(v)
    }} 
    pub type ArithmeticException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.ArithmeticException">;
    use super::*;
    impl From<ArithmeticException> for System::SystemException {
     fn from(v:ArithmeticException)->System::SystemException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::SystemException,ArithmeticException>(v)
    }} 
    pub type ArrayEnumerator =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.ArrayEnumerator">;
    use super::*;
    impl From<ArrayEnumerator> for System::Object {
     fn from(v:ArrayEnumerator)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,ArrayEnumerator>(v)
    }} 
    pub type SZGenericArrayEnumeratorBase =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.SZGenericArrayEnumeratorBase">;
    use super::*;
    impl From<SZGenericArrayEnumeratorBase> for System::Object {
     fn from(v:SZGenericArrayEnumeratorBase)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,SZGenericArrayEnumeratorBase>(v)
    }} 
    pub type GenericEmptyEnumeratorBase =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.GenericEmptyEnumeratorBase">;
    use super::*;
    impl From<GenericEmptyEnumeratorBase> for System::Object {
     fn from(v:GenericEmptyEnumeratorBase)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,GenericEmptyEnumeratorBase>(v)
    }} 
    pub type ArrayTypeMismatchException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.ArrayTypeMismatchException">;
    use super::*;
    impl From<ArrayTypeMismatchException> for System::SystemException {
     fn from(v:ArrayTypeMismatchException)->System::SystemException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::SystemException,ArrayTypeMismatchException>(v)
    }} 
    pub type AssemblyLoadEventArgs =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.AssemblyLoadEventArgs">;
    use super::*;
    impl From<AssemblyLoadEventArgs> for System::EventArgs {
     fn from(v:AssemblyLoadEventArgs)->System::EventArgs{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::EventArgs,AssemblyLoadEventArgs>(v)
    }} 
    pub type AssemblyLoadEventHandler =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.AssemblyLoadEventHandler">;
    use super::*;
    impl From<AssemblyLoadEventHandler> for System::MulticastDelegate {
     fn from(v:AssemblyLoadEventHandler)->System::MulticastDelegate{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::MulticastDelegate,AssemblyLoadEventHandler>(v)
    }} 
    pub type AsyncCallback =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.AsyncCallback">;
    use super::*;
    impl From<AsyncCallback> for System::MulticastDelegate {
     fn from(v:AsyncCallback)->System::MulticastDelegate{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::MulticastDelegate,AsyncCallback>(v)
    }} 
    pub type AttributeUsageAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.AttributeUsageAttribute">;
    use super::*;
    impl From<AttributeUsageAttribute> for System::Attribute {
     fn from(v:AttributeUsageAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,AttributeUsageAttribute>(v)
    }} 
    pub type BitConverter =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.BitConverter">;
    use super::*;
    impl From<BitConverter> for System::Object {
     fn from(v:BitConverter)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,BitConverter>(v)
    }} 
    pub type CannotUnloadAppDomainException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.CannotUnloadAppDomainException">;
    use super::*;
    impl From<CannotUnloadAppDomainException> for System::SystemException {
     fn from(v:CannotUnloadAppDomainException)->System::SystemException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::SystemException,CannotUnloadAppDomainException>(v)
    }} 
    pub type CharEnumerator =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.CharEnumerator">;
    use super::*;
    impl From<CharEnumerator> for System::Object {
     fn from(v:CharEnumerator)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,CharEnumerator>(v)
    }} 
    pub type CLSCompliantAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.CLSCompliantAttribute">;
    use super::*;
    impl From<CLSCompliantAttribute> for System::Attribute {
     fn from(v:CLSCompliantAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,CLSCompliantAttribute>(v)
    }} 
    pub type ContextBoundObject =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.ContextBoundObject">;
    use super::*;
    impl From<ContextBoundObject> for System::MarshalByRefObject {
     fn from(v:ContextBoundObject)->System::MarshalByRefObject{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::MarshalByRefObject,ContextBoundObject>(v)
    }} 
    pub type ContextMarshalException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.ContextMarshalException">;
    use super::*;
    impl From<ContextMarshalException> for System::SystemException {
     fn from(v:ContextMarshalException)->System::SystemException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::SystemException,ContextMarshalException>(v)
    }} 
    pub type ContextStaticAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.ContextStaticAttribute">;
    use super::*;
    impl From<ContextStaticAttribute> for System::Attribute {
     fn from(v:ContextStaticAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,ContextStaticAttribute>(v)
    }} 
    pub type Convert =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Convert">;
    use super::*;
    impl From<Convert> for System::Object {
     fn from(v:Convert)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Convert>(v)
    }} 
    pub type CurrentSystemTimeZone =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.CurrentSystemTimeZone">;
    use super::*;
    impl From<CurrentSystemTimeZone> for System::TimeZone {
     fn from(v:CurrentSystemTimeZone)->System::TimeZone{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::TimeZone,CurrentSystemTimeZone>(v)
    }} 
    pub type DataMisalignedException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.DataMisalignedException">;
    use super::*;
    impl From<DataMisalignedException> for System::SystemException {
     fn from(v:DataMisalignedException)->System::SystemException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::SystemException,DataMisalignedException>(v)
    }} 
    pub type DBNull =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.DBNull">;
    use super::*;
    impl From<DBNull> for System::Object {
     fn from(v:DBNull)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,DBNull>(v)
    }} 
    pub type DefaultBinder =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.DefaultBinder">;
    use super::*;
    impl From<DefaultBinder> for System::Reflection::Binder {
     fn from(v:DefaultBinder)->System::Reflection::Binder{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Reflection::Binder,DefaultBinder>(v)
    }} 
    pub type DivideByZeroException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.DivideByZeroException">;
    use super::*;
    impl From<DivideByZeroException> for System::ArithmeticException {
     fn from(v:DivideByZeroException)->System::ArithmeticException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::ArithmeticException,DivideByZeroException>(v)
    }} 
    pub type DllNotFoundException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.DllNotFoundException">;
    use super::*;
    impl From<DllNotFoundException> for System::TypeLoadException {
     fn from(v:DllNotFoundException)->System::TypeLoadException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::TypeLoadException,DllNotFoundException>(v)
    }} 
    pub type DuplicateWaitObjectException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.DuplicateWaitObjectException">;
    use super::*;
    impl From<DuplicateWaitObjectException> for System::ArgumentException {
     fn from(v:DuplicateWaitObjectException)->System::ArgumentException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::ArgumentException,DuplicateWaitObjectException>(v)
    }} 
    pub type Empty =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Empty">;
    use super::*;
    impl From<Empty> for System::Object {
     fn from(v:Empty)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Empty>(v)
    }} 
    pub type EntryPointNotFoundException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.EntryPointNotFoundException">;
    use super::*;
    impl From<EntryPointNotFoundException> for System::TypeLoadException {
     fn from(v:EntryPointNotFoundException)->System::TypeLoadException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::TypeLoadException,EntryPointNotFoundException>(v)
    }} 
    pub type EventArgs =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.EventArgs">;
    use super::*;
    impl From<EventArgs> for System::Object {
     fn from(v:EventArgs)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,EventArgs>(v)
    }} 
    pub type EventHandler =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.EventHandler">;
    use super::*;
    impl From<EventHandler> for System::MulticastDelegate {
     fn from(v:EventHandler)->System::MulticastDelegate{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::MulticastDelegate,EventHandler>(v)
    }} 
    pub type ExecutionEngineException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.ExecutionEngineException">;
    use super::*;
    impl From<ExecutionEngineException> for System::SystemException {
     fn from(v:ExecutionEngineException)->System::SystemException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::SystemException,ExecutionEngineException>(v)
    }} 
    pub type FieldAccessException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.FieldAccessException">;
    use super::*;
    impl From<FieldAccessException> for System::MemberAccessException {
     fn from(v:FieldAccessException)->System::MemberAccessException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::MemberAccessException,FieldAccessException>(v)
    }} 
    pub type FlagsAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.FlagsAttribute">;
    use super::*;
    impl From<FlagsAttribute> for System::Attribute {
     fn from(v:FlagsAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,FlagsAttribute>(v)
    }} 
    pub type FormatException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.FormatException">;
    use super::*;
    impl From<FormatException> for System::SystemException {
     fn from(v:FormatException)->System::SystemException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::SystemException,FormatException>(v)
    }} 
    pub type FormattableString =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.FormattableString">;
    use super::*;
    impl From<FormattableString> for System::Object {
     fn from(v:FormattableString)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,FormattableString>(v)
    }} 
    pub type GCMemoryInfoData =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.GCMemoryInfoData">;
    use super::*;
    impl From<GCMemoryInfoData> for System::Object {
     fn from(v:GCMemoryInfoData)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,GCMemoryInfoData>(v)
    }} 
    pub type Gen2GcCallback =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Gen2GcCallback">;
    use super::*;
    impl From<Gen2GcCallback> for System::Runtime::ConstrainedExecution::CriticalFinalizerObject {
     fn from(v:Gen2GcCallback)->System::Runtime::ConstrainedExecution::CriticalFinalizerObject{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Runtime::ConstrainedExecution::CriticalFinalizerObject,Gen2GcCallback>(v)
    }} 
    pub type DateTimeFormat =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.DateTimeFormat">;
    use super::*;
    impl From<DateTimeFormat> for System::Object {
     fn from(v:DateTimeFormat)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,DateTimeFormat>(v)
    }} 
    pub type DateTimeParse =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.DateTimeParse">;
    use super::*;
    impl From<DateTimeParse> for System::Object {
     fn from(v:DateTimeParse)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,DateTimeParse>(v)
    }} 
    pub type IAsyncDisposable =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.IAsyncDisposable">;
    use super::*;
    pub type IAsyncResult =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.IAsyncResult">;
    use super::*;
    pub type ICloneable =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.ICloneable">;
    use super::*;
    pub type IComparable =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.IComparable">;
    use super::*;
    pub type IConvertible =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.IConvertible">;
    use super::*;
    pub type ICustomFormatter =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.ICustomFormatter">;
    use super::*;
    pub type IDisposable =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.IDisposable">;
    use super::*;
    pub type IFormatProvider =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.IFormatProvider">;
    use super::*;
    pub type IFormattable =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.IFormattable">;
    use super::*;
    pub type IndexOutOfRangeException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.IndexOutOfRangeException">;
    use super::*;
    impl From<IndexOutOfRangeException> for System::SystemException {
     fn from(v:IndexOutOfRangeException)->System::SystemException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::SystemException,IndexOutOfRangeException>(v)
    }} 
    pub type InsufficientExecutionStackException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.InsufficientExecutionStackException">;
    use super::*;
    impl From<InsufficientExecutionStackException> for System::SystemException {
     fn from(v:InsufficientExecutionStackException)->System::SystemException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::SystemException,InsufficientExecutionStackException>(v)
    }} 
    pub type InsufficientMemoryException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.InsufficientMemoryException">;
    use super::*;
    impl From<InsufficientMemoryException> for System::OutOfMemoryException {
     fn from(v:InsufficientMemoryException)->System::OutOfMemoryException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::OutOfMemoryException,InsufficientMemoryException>(v)
    }} 
    pub type InvalidCastException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.InvalidCastException">;
    use super::*;
    impl From<InvalidCastException> for System::SystemException {
     fn from(v:InvalidCastException)->System::SystemException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::SystemException,InvalidCastException>(v)
    }} 
    pub type InvalidOperationException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.InvalidOperationException">;
    use super::*;
    impl From<InvalidOperationException> for System::SystemException {
     fn from(v:InvalidOperationException)->System::SystemException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::SystemException,InvalidOperationException>(v)
    }} 
    pub type InvalidProgramException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.InvalidProgramException">;
    use super::*;
    impl From<InvalidProgramException> for System::SystemException {
     fn from(v:InvalidProgramException)->System::SystemException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::SystemException,InvalidProgramException>(v)
    }} 
    pub type InvalidTimeZoneException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.InvalidTimeZoneException">;
    use super::*;
    impl From<InvalidTimeZoneException> for System::Exception {
     fn from(v:InvalidTimeZoneException)->System::Exception{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Exception,InvalidTimeZoneException>(v)
    }} 
    pub type ISpanFormattable =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.ISpanFormattable">;
    use super::*;
    pub type IUtf8SpanFormattable =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.IUtf8SpanFormattable">;
    use super::*;
    pub type LazyHelper =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.LazyHelper">;
    use super::*;
    impl From<LazyHelper> for System::Object {
     fn from(v:LazyHelper)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,LazyHelper>(v)
    }} 
    pub type LoaderOptimizationAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.LoaderOptimizationAttribute">;
    use super::*;
    impl From<LoaderOptimizationAttribute> for System::Attribute {
     fn from(v:LoaderOptimizationAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,LoaderOptimizationAttribute>(v)
    }} 
    pub type LocalAppContextSwitches =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.LocalAppContextSwitches">;
    use super::*;
    impl From<LocalAppContextSwitches> for System::Object {
     fn from(v:LocalAppContextSwitches)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,LocalAppContextSwitches>(v)
    }} 
    pub type LocalDataStoreSlot =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.LocalDataStoreSlot">;
    use super::*;
    impl From<LocalDataStoreSlot> for System::Object {
     fn from(v:LocalDataStoreSlot)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,LocalDataStoreSlot>(v)
    }} 
    pub type MarshalByRefObject =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.MarshalByRefObject">;
    use super::*;
    impl From<MarshalByRefObject> for System::Object {
     fn from(v:MarshalByRefObject)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,MarshalByRefObject>(v)
    }} 
    pub type Marvin =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Marvin">;
    use super::*;
    impl From<Marvin> for System::Object {
     fn from(v:Marvin)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Marvin>(v)
    }} 
    pub type MemberAccessException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.MemberAccessException">;
    use super::*;
    impl From<MemberAccessException> for System::SystemException {
     fn from(v:MemberAccessException)->System::SystemException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::SystemException,MemberAccessException>(v)
    }} 
    pub type MemoryExtensions =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.MemoryExtensions">;
    use super::*;
    impl From<MemoryExtensions> for System::Object {
     fn from(v:MemoryExtensions)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,MemoryExtensions>(v)
    }} 
    pub type MethodAccessException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.MethodAccessException">;
    use super::*;
    impl From<MethodAccessException> for System::MemberAccessException {
     fn from(v:MethodAccessException)->System::MemberAccessException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::MemberAccessException,MethodAccessException>(v)
    }} 
    pub type MissingFieldException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.MissingFieldException">;
    use super::*;
    impl From<MissingFieldException> for System::MissingMemberException {
     fn from(v:MissingFieldException)->System::MissingMemberException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::MissingMemberException,MissingFieldException>(v)
    }} 
    pub type MissingMemberException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.MissingMemberException">;
    use super::*;
    impl From<MissingMemberException> for System::MemberAccessException {
     fn from(v:MissingMemberException)->System::MemberAccessException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::MemberAccessException,MissingMemberException>(v)
    }} 
    pub type MissingMethodException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.MissingMethodException">;
    use super::*;
    impl From<MissingMethodException> for System::MissingMemberException {
     fn from(v:MissingMethodException)->System::MissingMemberException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::MissingMemberException,MissingMethodException>(v)
    }} 
    pub type MulticastNotSupportedException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.MulticastNotSupportedException">;
    use super::*;
    impl From<MulticastNotSupportedException> for System::SystemException {
     fn from(v:MulticastNotSupportedException)->System::SystemException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::SystemException,MulticastNotSupportedException>(v)
    }} 
    pub type NonSerializedAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.NonSerializedAttribute">;
    use super::*;
    impl From<NonSerializedAttribute> for System::Attribute {
     fn from(v:NonSerializedAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,NonSerializedAttribute>(v)
    }} 
    pub type NotFiniteNumberException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.NotFiniteNumberException">;
    use super::*;
    impl From<NotFiniteNumberException> for System::ArithmeticException {
     fn from(v:NotFiniteNumberException)->System::ArithmeticException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::ArithmeticException,NotFiniteNumberException>(v)
    }} 
    pub type NotImplementedException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.NotImplementedException">;
    use super::*;
    impl From<NotImplementedException> for System::SystemException {
     fn from(v:NotImplementedException)->System::SystemException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::SystemException,NotImplementedException>(v)
    }} 
    pub type NotSupportedException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.NotSupportedException">;
    use super::*;
    impl From<NotSupportedException> for System::SystemException {
     fn from(v:NotSupportedException)->System::SystemException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::SystemException,NotSupportedException>(v)
    }} 
    pub type Nullable =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Nullable">;
    use super::*;
    impl From<Nullable> for System::Object {
     fn from(v:Nullable)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Nullable>(v)
    }} 
    pub type NullReferenceException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.NullReferenceException">;
    use super::*;
    impl From<NullReferenceException> for System::SystemException {
     fn from(v:NullReferenceException)->System::SystemException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::SystemException,NullReferenceException>(v)
    }} 
    pub type Number =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Number">;
    use super::*;
    impl From<Number> for System::Object {
     fn from(v:Number)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Number>(v)
    }} 
    pub type ObjectDisposedException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.ObjectDisposedException">;
    use super::*;
    impl From<ObjectDisposedException> for System::InvalidOperationException {
     fn from(v:ObjectDisposedException)->System::InvalidOperationException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::InvalidOperationException,ObjectDisposedException>(v)
    }} 
    pub type ObsoleteAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.ObsoleteAttribute">;
    use super::*;
    impl From<ObsoleteAttribute> for System::Attribute {
     fn from(v:ObsoleteAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,ObsoleteAttribute>(v)
    }} 
    pub type OperatingSystem =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.OperatingSystem">;
    use super::*;
    impl From<OperatingSystem> for System::Object {
     fn from(v:OperatingSystem)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,OperatingSystem>(v)
    }} 
    pub type OperationCanceledException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.OperationCanceledException">;
    use super::*;
    impl From<OperationCanceledException> for System::SystemException {
     fn from(v:OperationCanceledException)->System::SystemException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::SystemException,OperationCanceledException>(v)
    }} 
    pub type OutOfMemoryException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.OutOfMemoryException">;
    use super::*;
    impl From<OutOfMemoryException> for System::SystemException {
     fn from(v:OutOfMemoryException)->System::SystemException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::SystemException,OutOfMemoryException>(v)
    }} 
    pub type OverflowException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.OverflowException">;
    use super::*;
    impl From<OverflowException> for System::ArithmeticException {
     fn from(v:OverflowException)->System::ArithmeticException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::ArithmeticException,OverflowException>(v)
    }} 
    pub type ParamArrayAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.ParamArrayAttribute">;
    use super::*;
    impl From<ParamArrayAttribute> for System::Attribute {
     fn from(v:ParamArrayAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,ParamArrayAttribute>(v)
    }} 
    pub type ParseNumbers =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.ParseNumbers">;
    use super::*;
    impl From<ParseNumbers> for System::Object {
     fn from(v:ParseNumbers)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,ParseNumbers>(v)
    }} 
    pub type PasteArguments =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.PasteArguments">;
    use super::*;
    impl From<PasteArguments> for System::Object {
     fn from(v:PasteArguments)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,PasteArguments>(v)
    }} 
    pub type PlatformNotSupportedException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.PlatformNotSupportedException">;
    use super::*;
    impl From<PlatformNotSupportedException> for System::NotSupportedException {
     fn from(v:PlatformNotSupportedException)->System::NotSupportedException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::NotSupportedException,PlatformNotSupportedException>(v)
    }} 
    pub type ProgressStatics =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.ProgressStatics">;
    use super::*;
    impl From<ProgressStatics> for System::Object {
     fn from(v:ProgressStatics)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,ProgressStatics>(v)
    }} 
    pub type Random =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Random">;
    use super::*;
    impl From<Random> for System::Object {
     fn from(v:Random)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Random>(v)
    }} 
    pub type RankException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.RankException">;
    use super::*;
    impl From<RankException> for System::SystemException {
     fn from(v:RankException)->System::SystemException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::SystemException,RankException>(v)
    }} 
    pub type ResolveEventArgs =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.ResolveEventArgs">;
    use super::*;
    impl From<ResolveEventArgs> for System::EventArgs {
     fn from(v:ResolveEventArgs)->System::EventArgs{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::EventArgs,ResolveEventArgs>(v)
    }} 
    pub type ResolveEventHandler =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.ResolveEventHandler">;
    use super::*;
    impl From<ResolveEventHandler> for System::MulticastDelegate {
     fn from(v:ResolveEventHandler)->System::MulticastDelegate{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::MulticastDelegate,ResolveEventHandler>(v)
    }} 
    pub type SerializableAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.SerializableAttribute">;
    use super::*;
    impl From<SerializableAttribute> for System::Attribute {
     fn from(v:SerializableAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,SerializableAttribute>(v)
    }} 
    pub type SpanHelpers =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.SpanHelpers">;
    use super::*;
    impl From<SpanHelpers> for System::Object {
     fn from(v:SpanHelpers)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,SpanHelpers>(v)
    }} 
    pub type PackedSpanHelpers =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.PackedSpanHelpers">;
    use super::*;
    impl From<PackedSpanHelpers> for System::Object {
     fn from(v:PackedSpanHelpers)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,PackedSpanHelpers>(v)
    }} 
    pub type SR =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.SR">;
    use super::*;
    impl From<SR> for System::Object {
     fn from(v:SR)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,SR>(v)
    }} 
    pub type StackOverflowException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.StackOverflowException">;
    use super::*;
    impl From<StackOverflowException> for System::SystemException {
     fn from(v:StackOverflowException)->System::SystemException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::SystemException,StackOverflowException>(v)
    }} 
    pub type StringComparer =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.StringComparer">;
    use super::*;
    impl From<StringComparer> for System::Object {
     fn from(v:StringComparer)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,StringComparer>(v)
    }} 
    pub type CultureAwareComparer =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.CultureAwareComparer">;
    use super::*;
    impl From<CultureAwareComparer> for System::StringComparer {
     fn from(v:CultureAwareComparer)->System::StringComparer{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::StringComparer,CultureAwareComparer>(v)
    }} 
    pub type OrdinalComparer =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.OrdinalComparer">;
    use super::*;
    impl From<OrdinalComparer> for System::StringComparer {
     fn from(v:OrdinalComparer)->System::StringComparer{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::StringComparer,OrdinalComparer>(v)
    }} 
    pub type OrdinalCaseSensitiveComparer =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.OrdinalCaseSensitiveComparer">;
    use super::*;
    impl From<OrdinalCaseSensitiveComparer> for System::OrdinalComparer {
     fn from(v:OrdinalCaseSensitiveComparer)->System::OrdinalComparer{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::OrdinalComparer,OrdinalCaseSensitiveComparer>(v)
    }} 
    pub type OrdinalIgnoreCaseComparer =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.OrdinalIgnoreCaseComparer">;
    use super::*;
    impl From<OrdinalIgnoreCaseComparer> for System::OrdinalComparer {
     fn from(v:OrdinalIgnoreCaseComparer)->System::OrdinalComparer{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::OrdinalComparer,OrdinalIgnoreCaseComparer>(v)
    }} 
    pub type StringNormalizationExtensions =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.StringNormalizationExtensions">;
    use super::*;
    impl From<StringNormalizationExtensions> for System::Object {
     fn from(v:StringNormalizationExtensions)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,StringNormalizationExtensions>(v)
    }} 
    pub type SystemException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.SystemException">;
    use super::*;
    impl From<SystemException> for System::Exception {
     fn from(v:SystemException)->System::Exception{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Exception,SystemException>(v)
    }} 
    pub type STAThreadAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.STAThreadAttribute">;
    use super::*;
    impl From<STAThreadAttribute> for System::Attribute {
     fn from(v:STAThreadAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,STAThreadAttribute>(v)
    }} 
    pub type MTAThreadAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.MTAThreadAttribute">;
    use super::*;
    impl From<MTAThreadAttribute> for System::Attribute {
     fn from(v:MTAThreadAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,MTAThreadAttribute>(v)
    }} 
    pub type ThreadStaticAttribute =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.ThreadStaticAttribute">;
    use super::*;
    impl From<ThreadStaticAttribute> for System::Attribute {
     fn from(v:ThreadStaticAttribute)->System::Attribute{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Attribute,ThreadStaticAttribute>(v)
    }} 
    pub type ThrowHelper =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.ThrowHelper">;
    use super::*;
    impl From<ThrowHelper> for System::Object {
     fn from(v:ThrowHelper)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,ThrowHelper>(v)
    }} 
    pub type TimeoutException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.TimeoutException">;
    use super::*;
    impl From<TimeoutException> for System::SystemException {
     fn from(v:TimeoutException)->System::SystemException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::SystemException,TimeoutException>(v)
    }} 
    pub type TimeZone =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.TimeZone">;
    use super::*;
    impl From<TimeZone> for System::Object {
     fn from(v:TimeZone)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,TimeZone>(v)
    }} 
    pub type TimeZoneInfo =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.TimeZoneInfo">;
    use super::*;
    impl From<TimeZoneInfo> for System::Object {
     fn from(v:TimeZoneInfo)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,TimeZoneInfo>(v)
    }} 
    pub type TimeZoneNotFoundException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.TimeZoneNotFoundException">;
    use super::*;
    impl From<TimeZoneNotFoundException> for System::Exception {
     fn from(v:TimeZoneNotFoundException)->System::Exception{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Exception,TimeZoneNotFoundException>(v)
    }} 
    pub type ITupleInternal =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.ITupleInternal">;
    use super::*;
    pub type Tuple =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Tuple">;
    use super::*;
    impl From<Tuple> for System::Object {
     fn from(v:Tuple)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Tuple>(v)
    }} 
    pub type TupleExtensions =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.TupleExtensions">;
    use super::*;
    impl From<TupleExtensions> for System::Object {
     fn from(v:TupleExtensions)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,TupleExtensions>(v)
    }} 
    pub type TypeAccessException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.TypeAccessException">;
    use super::*;
    impl From<TypeAccessException> for System::TypeLoadException {
     fn from(v:TypeAccessException)->System::TypeLoadException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::TypeLoadException,TypeAccessException>(v)
    }} 
    pub type TypeInitializationException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.TypeInitializationException">;
    use super::*;
    impl From<TypeInitializationException> for System::SystemException {
     fn from(v:TypeInitializationException)->System::SystemException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::SystemException,TypeInitializationException>(v)
    }} 
    pub type TypeUnloadedException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.TypeUnloadedException">;
    use super::*;
    impl From<TypeUnloadedException> for System::SystemException {
     fn from(v:TypeUnloadedException)->System::SystemException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::SystemException,TypeUnloadedException>(v)
    }} 
    pub type UnauthorizedAccessException =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.UnauthorizedAccessException">;
    use super::*;
    impl From<UnauthorizedAccessException> for System::SystemException {
     fn from(v:UnauthorizedAccessException)->System::SystemException{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::SystemException,UnauthorizedAccessException>(v)
    }} 
    pub type UnhandledExceptionEventArgs =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.UnhandledExceptionEventArgs">;
    use super::*;
    impl From<UnhandledExceptionEventArgs> for System::EventArgs {
     fn from(v:UnhandledExceptionEventArgs)->System::EventArgs{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::EventArgs,UnhandledExceptionEventArgs>(v)
    }} 
    pub type UnhandledExceptionEventHandler =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.UnhandledExceptionEventHandler">;
    use super::*;
    impl From<UnhandledExceptionEventHandler> for System::MulticastDelegate {
     fn from(v:UnhandledExceptionEventHandler)->System::MulticastDelegate{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::MulticastDelegate,UnhandledExceptionEventHandler>(v)
    }} 
    pub type UnitySerializationHolder =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.UnitySerializationHolder">;
    use super::*;
    impl From<UnitySerializationHolder> for System::Object {
     fn from(v:UnitySerializationHolder)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,UnitySerializationHolder>(v)
    }} 
    pub type IValueTupleInternal =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.IValueTupleInternal">;
    use super::*;
    pub type Version =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.Version">;
    use super::*;
    impl From<Version> for System::Object {
     fn from(v:Version)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Version>(v)
    }} 
    pub type WeakReference =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.WeakReference">;
    use super::*;
    impl From<WeakReference> for System::Object {
     fn from(v:WeakReference)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,WeakReference>(v)
    }} 
    pub type TimeProvider =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.TimeProvider">;
    use super::*;
    impl From<TimeProvider> for System::Object {
     fn from(v:TimeProvider)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,TimeProvider>(v)
    }} 
    pub type NotImplemented =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.NotImplemented">;
    use super::*;
    impl From<NotImplemented> for System::Object {
     fn from(v:NotImplemented)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,NotImplemented>(v)
    }} 
    pub type HexConverter =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","System.HexConverter">;
    use super::*;
    impl From<HexConverter> for System::Object {
     fn from(v:HexConverter)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,HexConverter>(v)
    }} 
    }
    pub mod Internal{
    pub mod Runtime{
    pub mod InteropServices{
    pub type ComponentActivator =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","Internal.Runtime.InteropServices.ComponentActivator">;
    use super::super::super::*;
    impl From<ComponentActivator> for System::Object {
     fn from(v:ComponentActivator)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,ComponentActivator>(v)
    }} 
    pub type ComActivator =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","Internal.Runtime.InteropServices.ComActivator">;
    use super::super::super::*;
    impl From<ComActivator> for System::Object {
     fn from(v:ComActivator)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,ComActivator>(v)
    }} 
    pub type InMemoryAssemblyLoader =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","Internal.Runtime.InteropServices.InMemoryAssemblyLoader">;
    use super::super::super::*;
    impl From<InMemoryAssemblyLoader> for System::Object {
     fn from(v:InMemoryAssemblyLoader)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,InMemoryAssemblyLoader>(v)
    }} 
    pub type IsolatedComponentLoadContext =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","Internal.Runtime.InteropServices.IsolatedComponentLoadContext">;
    use super::super::super::*;
    impl From<IsolatedComponentLoadContext> for System::Runtime::Loader::AssemblyLoadContext {
     fn from(v:IsolatedComponentLoadContext)->System::Runtime::Loader::AssemblyLoadContext{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Runtime::Loader::AssemblyLoadContext,IsolatedComponentLoadContext>(v)
    }} 
    }
    }
    pub type Console =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","Internal.Console">;
    use super::*;
    impl From<Console> for System::Object {
     fn from(v:Console)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Console>(v)
    }} 
    }
    pub type Interop =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","Interop">;
    impl From<Interop> for System::Object {
     fn from(v:Interop)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,Interop>(v)
    }} 
    pub type InteropErrorExtensions =  crate::intrinsics::RustcCLRInteropManagedClass<"System.Private.CoreLib","InteropErrorExtensions">;
    impl From<InteropErrorExtensions> for System::Object {
     fn from(v:InteropErrorExtensions)->System::Object{
    crate::intrinsics::rustc_clr_interop_managed_checked_cast::<System::Object,InteropErrorExtensions>(v)
    }} 

    