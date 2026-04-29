#!/usr/bin/env perl
# Fetch the upstream Material icons metadata, keep entries that are
# supported by "Material Symbols Outlined", and emit
# `assets/MaterialSymbolsOutlined.categories` as one
# `<category>\t<icon_name>` per line. Unicode categories like "av",
# "action", "navigation" come straight from Google's data.
#
# Run from the crate root:
#     perl scripts/fetch_categories.pl
#
# The output file is checked in; the gallery loads it at startup.
# Re-run only when the upstream metadata changes (new icons, or
# Google reshuffles categories).

use strict;
use warnings;
use JSON::PP;

my $url = 'https://fonts.google.com/metadata/icons?incomplete=true';
my $tmp = '/tmp/material_meta.json';

my $rc = system('curl', '-fsSL', $url, '-o', $tmp);
die "curl failed: $rc\n" if $rc != 0;

open my $fh, '<', $tmp or die "open $tmp: $!";
my $raw = do { local $/; <$fh> };
close $fh;

# Strip XSSI-protection prefix `)]}'\n` if present.
$raw =~ s/^\)\]\}'\s*//;

my $data = decode_json($raw);
my $icons = $data->{icons} or die "no 'icons' key";

# Bucket icon names by category. An icon can appear in multiple
# categories — we keep all of them (so it shows up under each tab).
my %by_cat;
my $kept = 0;
for my $entry (@$icons) {
    my $unsupported = $entry->{unsupported_families} || [];
    my %u = map { $_ => 1 } @$unsupported;
    next if $u{'Material Symbols Outlined'};
    my $name = $entry->{name} or next;
    my $cats = $entry->{categories} || ['uncategorized'];
    for my $cat (@$cats) {
        push @{$by_cat{$cat}}, $name;
    }
    $kept++;
}

# Stable order: alphabetical category, alphabetical icon within.
my @lines;
for my $cat (sort keys %by_cat) {
    my @names = sort { $a cmp $b } @{$by_cat{$cat}};
    # De-dupe within a category — rare but possible if an icon was
    # listed twice upstream.
    my %seen;
    @names = grep { !$seen{$_}++ } @names;
    for my $n (@names) {
        push @lines, "$cat\t$n";
    }
}

my $out_path = 'assets/MaterialSymbolsOutlined.categories';
open my $out, '>', $out_path or die "open $out_path: $!";
print $out "$_\n" for @lines;
close $out;

printf STDERR "Wrote %d (category, name) pairs across %d categories to %s\n",
    scalar(@lines), scalar(keys %by_cat), $out_path;
printf STDERR "Kept %d icons supported by 'Material Symbols Outlined'.\n", $kept;
