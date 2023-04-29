Name:           vmod-unleash
Version:        0.1.0
Release:        1%{?dist}
Group:          System Environment/Libraries
URL:            https://github.com/teqm/libvmod-unleash
Summary:        Unleash module for Varnish Cache

License:        BSD
Source0:        %{name}-%{version}.tar.gz

%description
Unleash module for Varnish Cache

%prep
%setup -n libvmod-unleash

%build
cargo build --release
$(pkg-config --variable=vmodtool varnishapi) vmod.vcc
rst2man vmod_unleash.man.rst > vmod_unleash.3

%install
install -D target/release/libvmod_unleash.so %{buildroot}%{_libdir}/varnish/vmods/libvmod_unleash.so
install -D vmod_unleash.3 "%{buildroot}%{_mandir}/man3/vmod_unleash.3"

%clean
rm -rf %{buildroot}

%files
%{_libdir}/varnish/vmods/
%{_mandir}/man3/

%changelog
